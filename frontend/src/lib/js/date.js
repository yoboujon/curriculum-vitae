export function formatDate(date) {
    const options = { day: 'numeric', month: 'long', year: 'numeric' };
    const cool_date = new Date(date);
    const formattedDate = cool_date.toLocaleDateString('fr-FR', options);
    return formattedDate;
}

export function formatDateMobile(date) {
    const options = { month: 'numeric', year: 'numeric' };
    const cool_date = new Date(date);
    const formattedDate = cool_date.toLocaleString('fr-FR', options);
    return formattedDate;
}

export function formatMonth(date) {
    const options = { month: 'long', year: 'numeric' };
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