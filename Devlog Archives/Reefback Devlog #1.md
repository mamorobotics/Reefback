**Devlog #1**

So far, I have been working on translating and reworking the architecture of networking code already present in previous projects into a new, easier format.
The major points of design have been increased interoperability of networking protocols between different hardware solutions and decreased systems overreach.

**Increased interoperability**
This has primarily come in the form of different _hardware packages,_ similar to the concept of dll's, wherein one piece of code is designed to function in different
ways depending on operation environment. As not every lower level operating system and hardware setup is the same, we can develop in such a way that interacting with
hardware and running protocol are on separate code spaces. Currently this is implemented by abstracting out `bind_to_address()`, which establishes a root connection
with a separate system, `send_to()`, which sends a message to a given separate system, and `recv()`, which waits for and returns the next received message.
By following this architecture, if say, implementing RF communications, you dont have to rewire the basic protocol, and instead just rewrite these functions following
the given standards. This allows for quicker and more robust development of congruent systems for different hardware that can all operate at the same time.

**Decreased overreach**
The second point that I have focused on is separating out the flow of control. We have done this by making the tools available at the end as powerful as possible while
still being limited in overreach. This means a _register_ system where we register specific functions for specific messages received, a simplified sending function, and
a simplified creation function. Together, these systems work together to most efficiently maximize what we can do, limiting the necessity to go into the protocol and
modify to get a system to work.

**Work still to be done on networking**
I still need to implement the host side of the handshake process, with currently only the client side being implemented, however this is a relatively painless step that
only requires a little translation of `C++` to `Rust`. I would also like to look into making the receiving function more powerful, with possibly multi-function
per header functionality, but as it stands, each unique header has a separate function that manages how data is parsed after it is received.

**Other notes**
Using the previously stated _hardware packages_ I have been able to create a _simulation_ hardware package that simulates data flows without actually requiring an active
internet connection with a host, however it is still in its infancy and I would really like to implement something more active in how it operates. This does however show
a benefit of this approach to rapid iteration and design changes.
