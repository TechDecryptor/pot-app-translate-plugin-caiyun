async function translate(text, from, to, options) {
    const { utils } = options;
    const { tauriFetch: fetch } = utils;

    const URL = "https://interpreter.cyapi.cn/v1/translator";

    const body = {
        "source": text,
        "detect": true,
        "os_type": "ios",
        "device_id": "F1F902F7-1780-4C88-848D-71F35D88A602",
        "trans_type": `${from}2${to}`,
        "media": "text",
        "request_id": Date.now() % 1000000000,
        "user_id": "",
        "dict": true
    };
    const headers = {
        "Content-Type": "application/json",
        "x-authorization": "token ssdj273ksdiwi923bsd9",
        "user-agent": "caiyunInterpreter/5 CFNetwork/1404.0.5 Darwin/22.3.0"
    };

    let res = await fetch(URL, {
        method: 'POST',
        headers: headers,
        body: {
            type: 'Json',
            payload: body
        },
    });

    if (res.ok) {
        let result = res.data;
        const { target } = result;
        if (target) {
            return target;
        } else {
            throw JSON.stringify(result.trim());
        }
    } else {
        throw `Http Request Error\nHttp Status: ${res.status}\n${JSON.stringify(res.data)}`;
    }
}
