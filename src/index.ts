import fs from "node:fs/promises";
import path from "node:path";
import { DISCORD_TOKEN, EVENT_FOLDER } from "./constants";
import { discord } from "./services";
import type { DiscordEventListener } from "./util/createDiscordEventListener";
import { log } from "./logger";

// load src/events
for (const fileName of await fs.readdir(EVENT_FOLDER)) {
	const {
		default: { event, once, listener },
	} = (await import(path.join(EVENT_FOLDER, fileName))) as { default: DiscordEventListener };

	if (once) {
		discord.once(event, listener);
	} else {
		discord.on(event, listener);
	}
}

await discord.login(DISCORD_TOKEN);

log.info(`Logged in as ${discord.user?.tag}`);
