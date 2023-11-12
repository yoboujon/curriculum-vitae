// processdata.js
export function processData(data) {
    if (data.status === 0) {
      const info = data.info[0];
      const experiences = data.experience;
      const education = data.education;
      const skills = data.skills[1];
      const projects = data.skills[0];
  
      return { info, experiences, education, skills, projects };
    } else {
      return null; // Indicates an error
    }
  }
  