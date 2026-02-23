const USE_WASM = import.meta.env.VITE_USE_WASM === '1' || import.meta.env.VITE_USE_WASM === 'true';

import { writable } from 'svelte/store';
import { eval_input } from '$lib/wasm';

type AppState = {
    history: Array<ServerMessage>,
};

function createGlobalState() {
    const { subscribe, set, update } = writable<{
        data: AppState;
        connected: boolean;
    }>({
        data: { history: [] } as AppState,
        connected: false,
    });

    let socket: WebSocket | undefined;

    async function connectWasm() {
        update(s => ({ ...s, connected: true }));

        return {
            send: async (msg: string) => {
                const result = await eval_input(msg);

                const parsed = typeof result === 'string' ? JSON.parse(result) : result;

                update(s => {
                    s.data.history.push(parsed);
                    return { ...s, connected: true };
                });
            }
        };
    }

    function connectWs() {
        const protocol = location.protocol === 'https:' ? 'wss' : 'ws';
        const url = `${protocol}://${location.host}/ws`;
        socket = new WebSocket(url);

        socket.onopen = () => update(s => ({ ...s, connected: true }));

        socket.onmessage = (event) => {
            try {
                const msg = JSON.parse(event.data);
                update(s => {
                    s.data.history.push(msg);
                    return { ...s, connected: true };
                });
            } catch (e) {
                console.log('Error parsing message:', e);
            }
        };

        socket.onclose = () => {
            update(s => ({ ...s, connected: false }));
            setTimeout(connectWs, 3000);
        };

        return {
            send: (msg: string) => socket?.send(msg),
        };
    }

    let transport: { send: (msg: string) => any } = { send: () => { } };

    if (typeof window !== 'undefined') {
        if (USE_WASM) {
            connectWasm().then(t => (transport = t));
        } else {
            transport = connectWs();
        }
    }

    return {
        subscribe,
        send: (msg: string) => transport.send(msg),
    };
}

export const appState = createGlobalState();
