import { locale_lang } from "$lib/js/date.js"

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
      const data = import.meta.glob('/src/lib/lang/*.json');
      const file = data[`/src/lib/lang/${lang}.json`];
      const jsonData = (await file()).default;
      return {
        status: 0,
        data: jsonData
      }
    } catch {
      return {
        status: 500,
      }
    }
  }

  // Updating locale
  const lang = context.params.lang;
  locale_lang.set(lang);
  // Gathering language id
  let lang_id;
  const res = (await fetchData(`get_lang_id/${lang}`));
  if (res.status == 500) return {
    status: res.status
  }
  else {
    lang_id = res.data.id;
  }
  // Updating json for locale text
  let text;
  const jsonData = await fetchJSON(lang);
  if (jsonData.status == 0) text = jsonData.data;
  else {
    return {
      status: res.status
    }
  }

  // Gathering data from database
  const infos = [];
  const dataToGather = [
    `info/${lang_id}`, `education/${lang_id}`, `experience/${lang_id}`,
    `project/${lang_id}`, 'hard_skills', 'tags', `getproject_programming/${lang_id}`, `categories/${lang_id}`
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

  return {
    status: 0,
    lang: lang,
    info: infos[0][0],
    education: infos[1],
    experience: infos[2],
    project: infos[3],
    languages: infos[4][0],
    skills: infos[4][1],
    tags: infos[5],
    project_skills: infos[6],
    categories: infos[7],
    text: text,
  };
}
