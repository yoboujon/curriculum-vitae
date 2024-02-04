function arrangeById(array) {
  let newArray = new Array(array.length);
  for (const value of array) {
    newArray[value.id-1] = value;
  }
  return newArray;
}

export function processData(data) {
  if (data.status === 0) {
    const info = data.info[0];
    const experiences = data.experience;
    const education = data.education;
    const skills = data.skills;
    const tags = data.tags;
    const project_programming = data.project_programming;
    const project_software = data.project_software;

    return {info, experiences, education, skills, tags, project_programming, project_software};
  } else {
    return null;  // Indicates an error
  }
}