# Ok Bloomer
First thing is world generation

Want to do based SoA game instead of cringe AoS

So 'opaque' is a thing
as its a bool we can have u64s.

64x64x64? no
128x128x64? maybe, z axis can be u64 so query above is if adjacent bits

but how to make it so god damn everything doesn go through the slow path
ping pong for everything and its one frame to the next?
duplication of stored data, chunks are myabe a few less but does not being a power of 2 fuck it?
but yea if its all neighbour based

yep chunks are powers of 2 but the mapping onto world coords is -1 from each side, so neighbours can be sampled fast
but it needs to be reconciled if a write occurs

and theres the complete information for the uniquely referenced values but what about the double ones that are shared by 2
separate slow pass?


Other ones for water (infiltrating) etc
moisture, etc


not only that but correctness at edge of world

i think chunks should just pull from their neighbours
and be 6-conn


and unloaded chunks, each chunk has sequence number, so when you load it can update up to where it should be, thats ok, might be kinda cheaty like duping water or something.

definitely dont want hyper breakage, probably better if things steady state more, like jank minecraft water

meshing on GPU with a geometry shader is a good option (not every frame)


any benefit to chunks, subchunks?
eg mesh at higher level
different chunk resolution for different things
could you do chunk edge bookkeeping? only really solves half the problem


using glMultiDrawElements and 