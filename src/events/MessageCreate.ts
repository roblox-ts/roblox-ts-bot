import Discord from "discord.js";
import { createDiscordEventListener } from "../util/createDiscordEventListener";
import { log } from "../logger";
import assert from "node:assert";

const PLAYGROUND_REGEX = /^\s*https:\/\/roblox-ts\.com\/playground\/#code\/[A-Za-z0-9\-+]+\s*$/;

export default createDiscordEventListener({
	event: Discord.Events.MessageCreate,
	listener: async event => {
		if (!PLAYGROUND_REGEX.test(event.content)) return;

		await event.channel.send({
			reply: event.reference?.messageId ? { messageReference: event.reference.messageId } : undefined,
			embeds: [
				{
					title: "Playground link",
					url: event.content.trim(),
					description: `Posted by <@${event.author.id}>`,
					color: 0xe2_24_1a,
				},
			],
		});

		await event.delete();

		// bot doesn't run in DMs, channel.name only works for non-DM channels
		assert(!event.channel.isDMBased());

		log.info({
			channnel: event.channel.name,
			author: event.author.tag,
			content: event.content,
		}, `Created embedded playground link for ${event.author.tag}`);
	},
});
