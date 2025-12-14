import Discord from "discord.js";
import { HELP_CHANNEL_ID, SOLVED_TAG_ID, UNSOLVED_TAG_ID } from "../constants";
import log from "@osyris/log";

export async function processThread(channel: Discord.AnyThreadChannel) {
	if (channel.parentId !== HELP_CHANNEL_ID) return;

	// force fetch latest data (including tags)
	channel = await channel.fetch();

	if (channel.archived) return;

	const hasSolved = channel.appliedTags.includes(SOLVED_TAG_ID);
	const hasUnsolved = channel.appliedTags.includes(UNSOLVED_TAG_ID);

	const { name, ownerId, appliedTags } = channel;
	const metadata = { name, ownerId, appliedTags, hasSolved, hasUnsolved };

	if (hasSolved && hasUnsolved) {
		log.info(`Removing unsolved tag from existing thread: "${name}"`, metadata);
		await channel.setAppliedTags(appliedTags.filter(tag => tag !== UNSOLVED_TAG_ID));
	} else if (!hasSolved && !hasUnsolved) {
		log.info(`Adding unsolved tag to existing thread: "${name}"`, metadata);
		await channel.setAppliedTags([...appliedTags, UNSOLVED_TAG_ID]);
	}
}
