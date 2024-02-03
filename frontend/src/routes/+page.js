import { redirect } from '@sveltejs/kit';

export async function load(context)
{
    redirect(301, '/lang=fr')
}