import { writable } from "svelte/store";

export let locale_lang = writable("en");

function getLocale(date)
{
    let locale;
    const unsubscribe = locale_lang.subscribe(value => {
        locale = value
    });
    unsubscribe();
    const cool_date = new Date(date);
    return {
        locale: locale,
        date: cool_date
    }
}

export function formatDate(date) {
    let data = getLocale(date);
    const options = { day: 'numeric', month: 'long', year: 'numeric' };
    return data.date.toLocaleDateString(data.locale, options);
}

export function formatDateMobile(date) {
    let data = getLocale(date);
    const options = { month: 'numeric', year: 'numeric' };
    return data.date.toLocaleDateString(data.locale, options);
}

export function formatMonth(date) {
    let data = getLocale(date);
    const options = { month: 'long', year: 'numeric' };
    return data.date.toLocaleDateString(data.locale, options);
}

export function formatYear(date) {
    let data = getLocale(date);
    const options = { year: 'numeric' };
    return data.date.toLocaleDateString(data.locale, options);
}