import {json} from '@sveltejs/kit';

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

  async function fetchJSON(lang) {
    try {
      const url = (import.meta.env.MODE === 'development') ? `lang/${lang}.json` :`static/lang/${lang}.json`;
      const resTemp = await context.fetch(url);
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

  // Gathering the language
  const lang = context.params.lang;
  const lang_id = (await fetchData(`get_lang_id/${lang}`)).data.id;

  // Gathering texts for languages
  const text = (await fetchJSON(lang)).data;

  // Gathering data from databse
  const infos = [];
  const project_software = [];
  const project_programming = [];
  const dataToGather = [
    `info/${lang_id}`, `education/${lang_id}`, `experience/${lang_id}`,
    `project/${lang_id}`, 'hard_skills', 'tags'
  ];
  for (const url of dataToGather) {
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

  // infos[4] = hardskills
  // infos[4][1] = Softwares
  for (let i = 0; i < infos[4][1].length; i++) {
    const res = await fetchData(`getproject_software/${i + 1}/${lang_id}`);
    if (res.status == 0) {
      project_software.push(res.data);
    } else {
      return {
        status: res.status
      }
    }
  }
  // infos[4][0] = Programming Languages
  for (let i = 0; i < infos[4][0].length; i++) {
    const res = await fetchData(`getproject_programming/${i + 1}/${lang_id}`);
    if (res.status == 0) {
      project_programming.push(res.data);
    } else {
      return {
        status: res.status
      }
    }
  }

  return {
    status: 0,
    lang: lang,
    info: infos[0],
    education: infos[1],
    experience: infos[2],
    skills: {
      project: infos[3],
      programming_languages: infos[4][0],
      softwares: infos[4][1],
      languages: infos[4][2],
    },
    tags: infos[5],
    project_programming: project_programming,
    project_software: project_software,
    text: text,
  };
}
