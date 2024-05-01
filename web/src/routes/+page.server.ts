/** @type {import('./$types').Actions} */
export const actions = {
	default: async (event) => {
		const data = await event.request.formData();
		const targetURL = data.get('link');
		// TODO: Send link shortening request to API
		// TODO: Respond to frontend with link
		return {
			link: `link = ${targetURL}`
		};
	}
};
