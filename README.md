# forthttp
**Forthttp is not a Forth Web Interpreter!** 

VIDEO QUICKSTART GUIDE:

https://drive.google.com/file/d/1xkBDqDxi48SamU2HKtBDSNsmShEg347N/view?usp=sharing

(OR)

Short Link:

https://youtu.be/tiKRZchBCKQ

(OR)

Long Link:

https://www.youtube.com/watch?v=tiKRZchBCKQ

TEXT QUICKSTART GUIDE:

https://github.com/txtatech/forthttp/blob/main/README_Quickstart_Guide.txt

(OR)

https://docs.google.com/document/d/1R7CBKz1g7bGoJvtMxYe0tsz6RxzlKeDVIqFFU17ztLE/edit?usp=sharing

(OR)

https://docs.google.com/document/d/1RFs70Qx1wmyuDe7HtkR18cOVL9SwWhtJ_HFDrESAVD4/edit?usp=sharing

### **About:**

**The 'forthttp' Forth Web Interpreter is a versatile software application that combines Rust-based web server functionality with Forth language integration. It allows users to interact with a web interface to input and execute Forth commands, providing a seamless and efficient way to work with Forth programming.**

Related Projects:

https://github.com/txtatech/ascii-qr-code-loader

https://github.com/txtatech/rusthttp

https://github.com/txtatech/static

https://github.com/txtatech/console-to-page

**System Architecture:**

Forthttp consists of the following key components:

**Browser:**

Interact with the application through a web browser, input Forth commands and receive the results of the execution.

**Rust Web Server:**

The Rust-based web server handles incoming requests from the browser, routes them to the appropriate endpoints, and communicates with the Forth runtime for command execution.

**Forth Runtime:**

The Forth runtime serves as the core engine that executes the Forth commands. It interprets and processes the commands, performs the necessary operations, and returns the results to the web server for further handling.

The architecture ensures a smooth flow of communication between the browser, web server, and Forth runtime, enabling users to execute Forth commands and receive immediate feedback through the web interface.

**QR Code Integration:**

## **Note:** **All JavaScript code loaded from QR codes is stored in the DOM, in a Blob, in Memory and as a Binary String.**

There are several components of Forthttp that handle QR codes in a variety of ways. 

On the main landing page there is a built-in ASCII QR code reader that loads the ASCII QR codes, scans their contents and executes any JavaScript code found in the ASCII QR codes if it is present.

On the images.html page (found in the 'static' folder) there is a QR code generation procedure executed that makes a code appear at the very top of the page with the phrase 'Hi!'. This can be modified for a variety of uses and acts as a placeholder for future features.

Also in the images.html file there is a QR Code Archive of .png files with a QR Code reader that handles that file extention. It may handle other extensions but I am unsure on that. This QR code reader requires the 'Cache QR Codes' link to be clicked to be initiated. I highly reccomend opening Developer Tools Console if you click the 'Cache QR Codes' link so that you can see the JavaScript (encoded as Base64) being scanned, 'cached' and made executable/acessible to the page and browser.

The code that gets loaded via the 'Cache QR Codes' link does a numner of things to setup a working enviroment for the site so that it can utilize service workers as well as other backend functions.

**Functionality and Usage**

The Forthttp offers a range of features and capabilities for executing Forth commands. You can access the web application through the browser and interact with the web interface. 

**Here's a step-by-step guide on how to use the application:**

**Accessing the Web Interface:** 

Open your preferred web browser and navigate to the URL where the Forthttp is hosted.

For example, you may access it at http://localhost:8080 or http://127.0.0.1:8080

**Input Forth Commands:**

In the provided input field on the web interface, enter your Forth commands. The interface offers a user-friendly environment for typing and managing your code.

**Execute the Commands:**

Once you've entered your Forth commands, click the "Execute" button on the web interface. The application will process the commands and send them to the Rust web server for execution.

**View Results:** 

The application will display the results of the executed Forth commands on the web interface. You can review the output and continue interacting with the system by entering more commands as desired.

In addition to the web interface, the Forthttp also provides an HTTP API, allowing developers to programmatically interact with the application. By sending HTTP POST requests to the appropriate endpoint (/forth), developers can execute Forth commands and retrieve the results programmatically.

**Installation and Setup:**

## **To set up the Forthttp, follow these steps:**

**Clone the Repository:**

Begin by cloning the repository containing the Forthttp codebase to your local machine using the following command:

git clone https://github.com/txtatech/forthttp.git

**Navigate to the Project Directory.** Move into the project directory by running the command:

cd forth-web-interpreter

**Build the Project:** Use Cargo, the Rust package manager, to build the project by executing the following command:

cargo build --release

**Start the Server:** Once the build process completes successfully, start the server by running the executable:

./target/release/forth-web-interpreter

This will start the web server, which will listen on localhost:8080 by default.

Now you have the Forthttp up and running, ready to accept Forth commands through the web interface or the HTTP API.

**More On QR Code Integration:
**

One of the notable features of the Forthttp is its QR Code integration. The application allows you to generate QR Codes that, when scanned, could direct users to a specific set of pre-loaded Forth commands. This functionality also enables dynamic loading and execution of JavaScript, providing a unique and interactive user experience.

**To utilize the QR Code integration:
**

Generate a QR Code: Use the provided functionality within the web interface to generate a QR Code. The QR Code will encode a URL that directs the user to the web interface, with the desired Forth commands included as a query parameter.

Scan the QR Code: Users can use their mobile devices or QR Code scanners to scan the generated QR Code. This action will open the web interface on their devices, with the Forth commands pre-loaded.

Execute the Pre-loaded Commands: Users can view the pre-loaded Forth commands on the web interface and execute them by clicking the "Execute" button. The results will be displayed in real-time, offering an interactive and engaging user experience.

The QR Code integration enhances the versatility and flexibility of Forthttp, allowing users to seamlessly load and execute Forth commands on-the-go which is a great advantage of using ASCII QR codes. Since they are represented in a binary format, they can be transmitted over the network with optimal efficiency. ASCII QR codes are compact and lightweight, consisting of a series of black and white pixels represented as characters. This allows for fast and efficient data transfer, especially when compared to transmitting larger image files.

By leveraging the binary representation of ASCII QR codes, Forthttp can deliver a seamless and responsive user experience. The compact size of ASCII QR codes makes them ideal for transmitting data in scenarios where bandwidth is limited or network speed is a concern.

Additionally, ASCII QR codes offer built-in error correction capabilities. The QR code specification includes error correction algorithms that allow for the accurate reconstruction of data even in the presence of damaged or partially obscured codes. This enhances the reliability of data transmission and ensures the integrity of the transmitted Forth commands.

Overall, the use of ASCII QR codes in Forthttp provides a fast, efficient, and reliable method for transmitting and executing Forth commands over the network. It enables quick interactions between users and the system, making it an excellent choice for scenarios where real-time communication and responsiveness are crucial.

**Security Considerations:**

When working with a web-based application like the Forthttp, it's crucial to prioritize security. Here are a few security considerations to keep in mind.

**User Input Sanitization:**

Ensure that all user input is properly sanitized before processing it on the server side. Apply appropriate input validation and filtering techniques to prevent code injection or other forms of malicious behavior.

Secure Communication: Protect the communication between the browser and the server by using secure protocols such as HTTPS. This helps to ensure data integrity and confidentiality during transmission.

Access Control: Implement proper access control mechanisms to restrict unauthorized access to the web application and its underlying resources. Consider user authentication and authorization techniques to enforce appropriate user privileges.

Code Review and Auditing: Regularly review the codebase for potential security vulnerabilities. Perform thorough security audits to identify and address any weaknesses in the application's design and implementation.

By adopting a proactive approach to security and following best practices, you can mitigate potential risks and ensure a secure and reliable environment for the Forthttp.

**Extensibility and Customization:**

Forthttp is built with flexibility and extensibility in mind, allowing you to tailor it to your specific needs. The Rust-based server provides a solid foundation for further development and customization. Here are some possibilities for extending and modifying the application:

Additional Functionality: You can expand the capabilities of the Forth runtime by incorporating new Forth words or implementing additional operations specific to your use case.

User Interface Enhancements: Customize the web interface to improve user experience, add visual elements, or incorporate other JavaScript libraries or frameworks.

Integration with External Systems: Utilize the HTTP API to integrate Forthttp with other systems or services, allowing seamless interoperability and expanding the scope of your application.

Optimizations and Performance Enhancements: Analyze the performance of the application and make optimizations where necessary. This may involve optimizing the Forth runtime, improving the Rust server code, or implementing caching mechanisms.

The extensibility and customization options enable you to adapt the Forthttp to suit your unique requirements and explore new possibilities in Forth programming.

## **Unconventional Uses and the Reverse Rust Kernel with Forth OS:**

Forthttp can serve as a foundation for exploring unconventional use cases and innovative solutions. Here are a few examples:

A Learning Tool: The integration of JavaScript, Rust, and Forth provides an excellent platform for studying low-level systems programming, operating system development, and programming language design. By analyzing the codebase and understanding the interactions between these components, developers and students can gain valuable insights into these areas of study.

Prototyping Hardware and Embedded Systems: With Rust's compatibility with embedded systems, Forthttp can be a valuable toolset for prototyping and testing custom hardware or novel embedded system designs. The ability to execute JavaScript and interface with the hardware opens up possibilities for rapid development and evaluation.

Networking and Text-based Interactions: Forthttp's RESTful API and text-based nature make it suitable for networking applications. It can be used as a minimalist communication platform, allowing devices to share information, exchange commands, or create a simple text-based social network.

Scripting and Automation: Leveraging Forthttp's API, developers can build custom automated workflows or task scripts. JavaScript code can be sent to the server to automate specific actions or integrate with other systems.

Game Development: By utilizing the QR code functionality and the ability to execute JavaScript, Forthttp can serve as a foundation for developing unique networked games or interactive experiences.

As for the Reverse Rust Kernel with a Forth OS, Forthttp provides a starting point for building a Rust-based kernel that interfaces with a Forth-based operating system. This approach can yield a lightweight and efficient system, suitable for resource-constrained environments. It opens up opportunities for exploring new programming paradigms, investigating security aspects, and building minimalist operating systems.

Forth's adaptability, extensibility, and unconventional uses make it a powerful tool for developers, researchers, and enthusiasts to explore new frontiers in software development and systems design.

**Conclusion:**

Forthttp is a feature-rich and flexible software application that enables seamless execution of Forth commands through a web interface. With its intuitive user interface, QR Code integration, and extensibility, it offers a unique and powerful platform for working with Forth programming. Whether you're a Forth enthusiast, a systems programmer, or a developer looking for innovative tools, Forthttp had much to offer. Explore its capabilities, experiment with its features, and unlock new possibilities in your programming journey.

### **Using the Web Interface:**

Navigate to localhost:8080 in your web browser to use the web interface. There, you will find an input field for entering your Forth commands. Once you've input your commands, click the 'Execute' button to process them. Results will appear in the output area below the input field.

## **HTTP API:**

In addition to the web interface, the server also exposes an HTTP API which can be used to execute Forth commands. To use this API, send an HTTP POST request to the /forth endpoint with a JSON body containing your commands.

# Example 1 using curl
curl -X POST -H "Content-Type: application/json" -d '{"commands":"1 2 + ."}' http://localhost:8080/forth

# Example 2 using curl
curl -X POST -H "Content-Type: application/json" -d '{"commands":"1 2 + ."}' http://127.0.0.1:8080/forth

This will execute the Forth command 1 2 + ., which adds the numbers 1 and 2 and outputs the result. The result of the command will be included in the response from the server.

**Advanced Usage:**

This application is designed to be flexible and highly interoperable. It can be integrated with other systems via its HTTP API, and its QR code functionality allows for dynamic loading and execution of JavaScript. Multiple instances of the application can be run simultaneously for increased throughput, and the application's design allows for easy integration into larger systems.

This application can be used as a unique and efficient tool for executing Forth commands via a web interface, whether for education, development, or any other purpose. Its simplicity, flexibility, and efficiency make it a powerful tool for any developer's toolkit.

## QR Code Integration:

Our application features an exciting QR Code integration. With this, you can generate a QR Code that, when scanned, directs a user to a specific set of Forth commands that are pre-loaded into the web interface. This is done by encoding the Forth commands into a QR Code which encodes a URL for the web interface, with the commands included as a query parameter.

# An example URL might look like this:

http://localhost:8080?commands=1%202%20%2B%20.

# Another example URL might look like this:

http://127.0.0.1:8080?commands=1%202%20%2B%20.

This functionality enables on-the-fly loading and execution of JavaScript, opening up a whole new world of possibilities for executing dynamic, mobile-friendly code with ease.

**Security:**

The JavaScript layer serves as a first-level security measure, sanitizing all user inputs before they're passed onto the Rust layer. This helps to prevent code injection and other types of attacks. However, as with all software, it is important to ensure that the server is secure and that appropriate measures are taken to prevent unauthorized access.

**Interfacing with Other Systems:**

The server exposes an HTTP API which makes it interoperable with other systems, including embedded systems. Even if the application can't run directly on an embedded system, it can still interface with it through the API.

# Example 1 Python code to interface with an embedded system
import requests

url = 'http://localhost:8080/forth'
data = {'commands': '1 2 + .'}

response = requests.post(url, json=data)

print(response.json())

# Example 2 Python code to interface with an embedded system
import requests

url = 'http://127.0.0.1:8080/forth'
data = {'commands': '1 2 + .'}

response = requests.post(url, json=data)

print(response.json())

As you can see, by utilizing these features, we can achieve a highly interactive and responsive Forth programming environment that can be adapted for a wide variety of scenarios.

## Networking and Multi-instance Deployment

The application's API is not just limited to a single instance. Multiple instances of the application can be deployed and networked together, enabling a distributed system of Forth servers that communicate and collaborate to solve complex tasks.

# Example of deploying multiple instances:

Instance 0 -> http://127.0.0.1:8080
Instance 1 -> http://localhost:8080
Instance 2 -> http://localhost:8081
Instance 3 -> http://localhost:8082

Each instance can serve a different set of tasks or they could share the load of processing a large number of commands.

**Extending and Modifying the Application:**

The written in Rust, which makes it easy to modify and extend the application with new features or to tailor it to specific use cases. If you're familiar with Rust, you can dive right into the codebase and start making modifications.

**Use Cases and Potential Applications:**

Given the application's design and capabilities, there are many potential uses for it. 

**Here are a few examples:**

Remote Code Execution: The server can be used to execute code remotely. This can be used in a wide variety of applications, from remote server administration to interactive programming competitions.

Dynamic Web Applications: Since the server integrates with JavaScript, it could be used to create dynamic, interactive web applications.

Mobile Applications: The QR Code functionality makes it easy to build mobile applications that execute Forth code on-the-go. Users can simply scan a QR Code to execute a set of commands.

Distributed Computing: As mentioned earlier, the server can be networked with other instances of itself to create a distributed computing system. This could be used for tasks that require large amounts of computing power, such as data processing or simulation.

Embedded Systems: While the server can't run directly on an embedded system, it can interface with them through the API. This could be used to control and manage embedded systems remotely.

## Security Considerations:

This application utilizes JavaScript execution, which inherently opens up potential security risks if not properly managed. JavaScript payloads received from untrusted sources should be thoroughly inspected and sanitized before execution to prevent any form of malicious behavior such as cross-site scripting (XSS) or remote code execution (RCE). 

# Simple Example: Sanitizing User Input
let sanitizedInput = userInput.replace(/<script[^>]*>([\S\s]*?)<\/script>/gmi, '');

It's vital to remember that when dealing with remote code execution, careful steps should be taken to secure your system, such as running the server in a sandboxed or containerized environment.

### **High-Level Benefits of The Software:**

Versatility: The application can be used in a variety of contexts due to its JavaScript integration and networkable architecture.

Scalability: The application can scale up or down easily due to its REST API and ability to deploy multiple instances.

Interactivity: The QR code feature offers a novel way for users to interact with the system.

Customizability: Given that the application is open source, it's fully customizable and adaptable to a variety of unique use cases.

Accessibility: The application can be accessed and controlled remotely due to its RESTful API.

Efficiency: The application's core is written in Rust, which offers excellent speed and memory safety.

## Unconventional Usages and The Reverse Rust Kernel with Its Forth OS

The software we've developed has potential beyond the conventional uses discussed earlier. It can serve as a starting point for developing a reverse Rust kernel with a Forth OS. Here are some interesting, less conventional use cases:

1. **A Learning Tool**: By studying how JavaScript is parsed and executed within a Rust environment, and how the Rust kernel interacts with the Forth OS, students and developers can gain a deeper understanding of low-level systems programming, operating systems, and programming languages.

2. **Prototyping Hardware and Embedded Systems**: Given the compatibility of Rust with many embedded systems, this software could be a valuable tool for prototyping and testing custom hardware or novel embedded system designs. The ability to execute JavaScript on such systems could offer a unique way to interface with the hardware.

3. **Networking**: The built-in REST API and QR code functionality open the door to some interesting networking applications. One could potentially use the system as a simple, text-based way to share information between devices, like a sort of minimalist social network.

4. **Scripting**: The combination of JavaScript execution and networked communication could lend itself to building custom automated workflows or task scripts. Users could send JavaScript code to a device running the software to automate certain actions.

5. **Game Development**: Given the interactivity of the QR code system and the ability to remotely execute JavaScript, one could use the software as a basis for developing unique, networked games.

**About the Reverse Rust Kernel with a Forth OS:**

The project as it stands can serve as a launchpad for developing a 'reverse' Rust kernel that interfaces with a Forth-based OS. This would involve extending the current functionality of the Rust layer to handle more system-level operations, such as memory management and hardware interfacing. The Forth OS, meanwhile, could be built on top of this reverse Rust kernel, providing a low-level, highly efficient platform for executing Forth code.

This opens up several opportunities:

1. **Building a Minimalist OS**: Forth is renowned for its simplicity and efficiency. A Forth OS built on a reverse Rust kernel could be a very lightweight and efficient system, suitable for resource-constrained environments.

2. **Exploring New Programming Paradigms**: Forth's stack-based, postfix syntax offers a different programming paradigm that might inspire new approaches to problem-solving.

3. **Investigating Security**: Given Rust's focus on safety and Forth's low-level nature, such a system could potentially offer new insights into secure operating system design.

Overall, the software offers a multitude of unconventional possibilities, making it a highly adaptable tool for various creative and innovative use cases.
