# Shared Memory For VieWS Processes
This provides an interface for having shared memory in VieWS processes so that
one can render graphics from one process to another.

The current goal of this is to render
[Powder Game](https://github.com/12Me21/powder-game-1) inside of VieWS.

Originally, I was going to have a TCP communication line between VieWS and
processes, but I feel like that's way too much effort because it's going to end
up with me having to end up creating support for other methods that I don't
really care about when all I want is the rendering.

Maybe even the SHM should exist purely inside of Powder Game because I also
want to be able to communicate actions like key inputs and mouse inputs.
I'll decide eventually I suppose.