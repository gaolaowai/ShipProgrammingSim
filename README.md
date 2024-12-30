# ShipProgrammingSim
An attempt to make a simple ship simulator... it's still up for debate, but "ship" here is likely going to be spaceship relying on newtonian mechanics in 3d space, and "programming" is going to be the ability/option to program the hardware modules which are plugged into your ship, such as scanners, radios, thrusters, claws, collectors, shields, etc.

When you acquire basic hardware modules or a basic ship, you start with a basic firmware programmed in a small assembly language (google TIS-100 computer game). Different modules communicate over a signal bus and/or message bus and queues.

After finishing the foundation of this programming system, add other system mechanics such as random bit flipping during solar storms or fights requiring either hardened modules or triplicate computation, as is required in real-life systems.

Also trying to avoid Clark Tech and keep things constrained to known physics, though maybe I'll allow limited cheating with the risk of death. For example, if you *somehow* modify your modules' firmware to allow all your fuel to be combusted at the same time, *maybe* you'll get an extra boost or maybe you'll just explode. Or in the case of client-server cheating... if you, as a local client application talking to a multiplayer server try to cheat, modifying your data packets, saying that you're traveling faster than normal accelleration physics, maybe have the server roll a dice to determine if while accellerating at 0.3% lightspeed, you've hit into a microastroid resulting in a small nuclear-explosion-sized magnitude explosion... so you can cheat, but it comes with calculated risks.

May allow for programming probes, relays, and other robotic systems for deployment around whichever star system, or writing the logic for systems on a space stations, trading posts, or defense platforms... we'll see.

## references
oort.rs --> I don't want to use any particular high-level programming language, but this was neat
TIS-100 --> This and sibling games by its creator were pretty inspiring for the idea behind this project.
Crafting Interpreters https://craftinginterpreters.com --> read it online, or get a hard copy. Absolutely worth it for understanding how to create interpreters and improving C programming as well.
