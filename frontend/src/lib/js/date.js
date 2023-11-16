export function formatDate(date) {
    const options = { day: 'numeric', month: 'long', year: 'numeric' };
    const cool_date = new Date(date);
    const formattedDate = cool_date.toLocaleDateString('fr-FR', options);
    return formattedDate;
}

export function formatYear(date) {
    const options = { year: 'numeric' };
    const cool_date = new Date(date);
    const formattedDate = cool_date.toLocaleDateString('fr-FR', options);
    return formattedDate;
}