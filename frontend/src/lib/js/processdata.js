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
    const experiences = arrangeById(data.experience);
    const education = arrangeById(data.education);
    const skills = data.skills[1];
    const projects = data.skills[0];

    return {info, experiences, education, skills, projects};
  } else {
    return null;  // Indicates an error
  }
}