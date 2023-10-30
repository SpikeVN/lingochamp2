LingoChamp Source Tree
======================
Open source development of the new Rust engine, dedicated to LingoChamp,
replacing the old SDL Python engine.

Technical specification
-----------------------
The server software will host a web service and a Socket.IO server on a 
hard-coded IP adress and port at /.

The protocol in which the client and server communicates in is currently 
under development.

Development
-----------
Install Rust using rustup and use cargo to run the server.
Frontend development takes place in the templates/ directory.
We are planning the migration from vanilla JavaScript to Svelte.

Licensing & Copyright
---------------------
Copyright (c) 2022-2023 EpiX Team CBN.

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
