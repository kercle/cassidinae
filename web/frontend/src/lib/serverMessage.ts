type ServerMessage = {
    evalResult: {
        input: string,
        output: string,
    }
} | {
    parseError: {
        input: string,
        msg: string,
    }
};
