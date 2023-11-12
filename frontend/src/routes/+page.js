export async function load(context) {
    const res = await context.fetch("http://0.0.0.0:8000/info");
    const info = await res.json();

    if (!res.ok) {
        return {
            status: 500,
            error: new Error("Internal Server Error"),
        };
    }
    return {
        info,
    };
}
