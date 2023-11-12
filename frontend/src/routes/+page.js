export async function load(context) {
    async function fetchData(data) {
        try {
            const resTemp = await context.fetch(`http://0.0.0.0:8000/${data}`);
            if(resTemp.ok == false) {
                return {
                    status: resTemp.status,
                }
            }
            return {
                status: 0,
                data: await resTemp.json(),
            }
        } catch(error) {
            return {
                status: 500,
            }
        }
    }

    const infos = [];
    const dataToGather = ["info", "education", "experience", "skills"];
    for (const url of dataToGather) {
        const res = await fetchData(url);
        if(res.status == 0)
        {
            infos.push(res.data);
        } else {
            return {
                status: res.status
            }
        }
    }
    
    return {
        status: 0,
        info: infos[0],
        education: infos[1],
        experience: infos[2],
        skills: infos[3]
    };
}
