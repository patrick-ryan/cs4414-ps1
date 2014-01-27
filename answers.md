Title: Problem Set 1 Answers
Author: Patrick Ryan

1.	User-Agent string:
		"Mozilla/5.0 (X11; Ubuntu; Linux i686; rv:26.0) Gecko/20100101 Firefox/26.0"

	Explanation: 
		"Mozilla/5.0" (now only used historically) specifies the type of user agent.
		In this case, the user agent is Mozilla version 5 compatible. 

		"X11; Ubuntu; Linux i686; rv:26.0" identifies the display system as an X 
		Window System, using the Ubuntu Linux distribution OS, running on an Intel 
		Pentium Pro CPU, with a CVS branch tag of rv:26.0 (Gecko version).

		"Gecko/20100101 Firefox/26.0" signifies the presence of a Gecko engine, the 
		build date of the browser, and the name (Firefox) and version (26.0) of the 
		browser. 

2.	Rust thinks it is unsafe to modify a global variable (one that is mutable and 
	static) because doing so increases the likelihood of running into concurrency 
	issues, such as a race condition which is caused by an uncontrolled sequence of 
	events (where, for example, a variable is being concurrently modified).

