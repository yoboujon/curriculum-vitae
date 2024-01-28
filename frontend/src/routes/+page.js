export async function load(context) {
  async function fetchData(data) {
    try {
      const resTemp = await context.fetch(`http://0.0.0.0:8000/${data}`);
      if (resTemp.ok == false) {
        return {
          status: resTemp.status,
        }
      }
      return {
        status: 0, data: await resTemp.json(),
      }
    } catch (error) {
      return {
        status: 500,
      }
    }
  }

  const infos = [];
  const project_software = [];
  const project_programming = [];
  const dataToGather =
      ['info', 'education', 'experience', 'skills/1', 'tags/1'];
  for (const [index, url] of dataToGather.entries()) {
    const res = await fetchData(url);
    if (res.status == 0) {
      // Pushing data
      infos.push(res.data);
    } else {
      return {
        status: res.status
      }
    }
  }

  for (let i = 0; i < infos[3][2].length; i++) {
    const res =
        await fetchData(`getproject_software/${i + 1}`);
    if (res.status == 0) {
      project_software.push(res.data);
    } else {
      return {
        status: res.status
      }
    }
  }
  for (let i = 0; i < infos[3][1].length; i++) {
    const res =
        await fetchData(`getproject_programming/${i + 1}`);
    if (res.status == 0) {
      project_programming.push(res.data);
    }
    else {
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
    skills: {
      project: infos[3][0],
      programming_languages: infos[3][1],
      softwares: infos[3][2],
      languages: infos[3][3],
    },
    tags: infos[4],
    project_programming: project_programming,
    project_software: project_software,
  };
}
