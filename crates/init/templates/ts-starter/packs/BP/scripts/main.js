import { world } from "@minecraft/server";

world.afterEvents.worldLoad.subscribe(() => {
  world.sendMessage("run 'npx just build'");
});
