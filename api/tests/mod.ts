const private_identity: string = await fetch("https://dev-api.nebsplay.space/identity/create", {
    body: JSON.stringify({
        name: "Test"
    }),
    method: "POST",
    headers: {
        "Content-Type": "application/json"
    }
}).then(res => res.text());

console.log(private_identity)

const channel: any = await fetch("https://dev-api.nebsplay.space/channels/create", {
    body: JSON.stringify({
        name: "Test",
        identity_id: private_identity
    }),
    method: "POST",
    headers: {
        "Content-Type": "application/json"
    }
}).then(res => res.json());

console.log(channel)

const start: any = await fetch("https://dev-api.nebsplay.space/channels/start", {
    body: JSON.stringify({
        id: channel.id,
        identity_id: private_identity,
        sdp: ""
    }),
    method: "POST",
    headers: {
        "Content-Type": "application/json"
    }
}).then(res => res.json());

console.log(JSON.stringify(start.sdp))