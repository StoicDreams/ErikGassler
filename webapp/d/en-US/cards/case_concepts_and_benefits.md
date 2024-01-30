# Basic Concepts & Benefits for Continuous Agile Software Engineering (CASE)

```quote "warning"
A quick note: This page is a brief introduction to a collection of standards and practices that I have dubbed Continuous Agile Software Engineering, or CASE for short.

CASE represents the accumulated standards from my 15+ years developing software that has so far been proven through my real-world usage to be the most efficient and effective at developing top-quality software that meets my high standards in the shortest amount of time.

My recent work in teams at Microsoft, Floating Point Group, and Costco, who all used various interpretations of Scrum, DevOps, and Sprint based workflows, has further proven to me that using CASE methodologies for true agile development with continuous delivery workflows were easily in the range of 10 to 100 times more productive than Scrum/DevOps standards. A team of 1 developer using these practices and workflows has been shown to deliver more in 3 to 6 months than what these teams of 8 to 12 engineers were taking 2 to 6 years to develop.

And had these teams chosen to adopt CASE standards for software development, they would have been enabled to not only deliver their products to consumers much faster, but they could also raise their quality standards in their work and products while doing so.

Check out my [book, CASE: Continuous Agile Software Engineering](https://amzn.to/3VzXW32), now available in Kindle, Paperback, and Hardcover editions on Amazon, as well as my software standards focused website [SoftwareStandards.dev](https://www.SoftwareStandards.dev) for more details on these [standards and practices for agile development using continuous delivery workflows](https://www.SoftwareStandards.dev).
```

```section
The CASE methodologies are practices and workflows that I have developed and refined over the course of my personal and professional career. And while many of the tenets and principles can be great on their own, I have found CASE works best when every tenet and core principle can be imlemented as a holistic and cohesive whole.

Professionally, I have been able to use the entirety of these CASE methodologies while developing software at General Services Corp between 2016 and 2018, where I lead multiple greenfield projects from inception through delivery and continued maintenance. And since that time I have worked on multiple teams with several variations of Scrum-based practices and workflows, including working on similar projects between the two sets of standards, so I have gotten to see a good apples-to-apples comparison between 1 engineer building a product using CASE versus 8+ engineers building a similar product using Scrum/DevOps.

Now, I want to be clear. I promote these standards because I value and prioritize productivity, code maintainability, security, user experiences, bug-free code, and fast delivery of updates and features. I know and accept other people will have other priorities. I have worked on many such teams, and when I do, I do my best to adapt and share my knowledge in ways that fit their priorities.

And sadly, I have encountered and even worked with several people that haven't liked me because I don't have the same opinions as they do. And I've been turned down from potential roles for the sole reason that I didn't share the same priorities as the people that interviewed me or were making the hiring decision. This makes me sad that these people are so close-minded to working with people for no other reason than they disagree with their opinions and values. Especially when they don't even take the time to ask questions to understand why someone would have a different opinion, to begin with.

If you are interested in prioritizing delivering higher quality software with faster releases, the below standards are a good place to start. Of course, exact adherence to all of these standards requires an environment where the management trusts their software engineers to not be micromanaged, as well as software engineers that are motivated to do good work.
```

## Twelve Tenets of Continuous Agile Software Engineering

```section
The CASE methodologies are broken up into twelve tenets, with each tenet containing three core principles.
```

`````section

````cards
```card "Tenet 1: Keep it Simple" "580" "success"
Keeping things simple means that software development should focus on delivering value in the simplest viable way. This is achieved by minimizing complexity in code, processes, and tools.

Keeping things simple needs to be both a mentality and a habit, and is aimed at helping to reduce the cost of development, improve quality of both the code and software, and reduce the time to market. It also helps to keep the development process agile, as simpler code and processes are easier to change and adapt to new requirements.

#### 1: Minimize Complexity

Avoid unnecessary complexity in code, processes, and tools. This helps to reduce the cost of development, improve quality throughout, and reduce the time to market.

#### 2: Eliminate Waste

Eliminate waste in the development process by removing unnecessary tasks, documentation, and other activities that do not add value. Eliminate code that is not being used and code comments that not adding value.

#### 3: Prioritize Value

Prioritize value over other factors such as features, aesthetics, and performance. This helps to ensure that development efforts are focused on delivering the most key features first. But also understand value doesn't have to be monetarily based or exclusive to customers / users. Some value may be for developers, stakeholders, or others as well.
```

```card "Tenet 2: Code Ownership" "580" "success"
Code Ownership is the concept that each repository within a team should have a designated owner responsible for its maintenance and improvement over time. The following are three core principles that can guide the implementation of Code Ownership:

#### Assign Ownership of Code

Each code repository should have a designated owner responsible for its overall quality, maintenance, and improvement over time. This owner should be the primary point of contact for all matters related to the code within that repository and have the authority to make decisions about its evolution, including code formatting and organization.

#### Foster Collaboration Among Owners

Ownership does not mean isolation. Code owners should collaborate with each other to share ideas and concepts and learn from each other's different perspectives and opinions. They should communicate frequently and openly to avoid redundancy, minimize conflicts, and make sure everyone is aligned with the business and team goals.

#### Rotate Ownership Over Time

Code ownership is not a lifelong appointment. As projects evolve, so do the skills, interests, and availability of team members. To ensure that code remains fresh and up-to-date, and owners do not grow stagnant, ownership should rotate periodically. This approach fosters a sense of shared responsibility and prevents bottlenecks that can occur when a single person is responsible for too much code.
```

```card "Tenet 3: Continuous Agility" "580" "success"
Continuous Agility means that teams should continuously adapt to changing requirements and market needs and be able to pivot quickly when necessary.

Continuous agility helps to ensure that teams can respond to new challenges and opportunities, and that they are always focused on delivering value to their customers.

#### Never follow with Absolution

When working within principles, guidelines, best- practices, never constrain yourself to following them with absolute adherence. There is no one- size-fits-all solution that is the best solution for every situation.

#### Be Customer-Focused

Maintain a customer-focused mindset and continuously seek feedback to ensure that products are meeting their needs and expectations and adapt tasks based on the feedback loops.

#### No Estimates or Time-Boxing

Break down projects and features into manageable tasks, but no task, feature, or project should be given deadlines or require developer estimates prior to or during active development.
```

```card "Tenet 4: Continuous Feedback Loops" "580" "success"
Establishing continuous feedback loops throughout the software development process means that developers and teams should continuously gather feedback from their IDEs, compilers, analytics, stakeholders, customers, and team members, and use that feedback to refine the development process and features being delivered.

Continuous feedback loops help teams and developers to stay focused on delivering value to customers, and to identify and address issues as early as possible in the development process. This helps to reduce the cost of development, improve quality, and increase customer satisfaction.

#### Gather Feedback

Continuously gather feedback from their tools, analytics, stakeholders, customers, and team members throughout the development process.

#### Use Feedback to Inform Decisions

Use feedback to inform decisions about features, design, and other aspects of the development process.

#### Iterate Based on Feedback
Continuously iterate on the development process based on feedback to improve the product and development process.
```

```card "Tenet 5: Continuous Automation" "580" "success"
Establishing continuous automation throughout the software development process means that teams should continuously automate repetitive and time- consuming tasks, such as testing, building, and deployment.

Continuous automation helps to reduce the time and effort required for these tasks, which in turn helps to increase productivity, improve quality, and reduce the time to market.

#### Automate Everything Possible

Identify and automate every task that can be automated, such as testing, building, and deployment.

#### Integrate Automation into the Development Process

Integrate automation tools and processes into the development process to reduce manual effort and increase efficiency. Always make this a top priority to do this as early as possible.

#### Continuously Improve Automation

Continuously improve automation processes to ensure that they are reliable, efficient, and effective.
```

```card "Tenet 6: Continuous Planning" "580" "success"
Establishing continuous planning throughout the software development process means that teams should continuously plan and adjust the development process based on feedback and changing requirements. This also means that teams should spend minimal time planning development or features that are not being immediately worked on.

Continuous planning helps teams to stay aligned with customer needs and market trends, and to make timely adjustments to the development process as needed. This helps to reduce the risk of developing products that do not meet customer needs or fail in the market.

#### Emphasize Continuous Planning

Prioritize planning throughout the development process, from initial requirements gathering to release planning.

#### Use Data-Driven Approaches

Use data-driven approaches, such as user feedback, A-B testing, and market trends, to inform planning decisions and adjust the development process.

#### Continuously Refine Plans

Continuously refine plans based on feedback, discoveries made during development, and changing requirements, and communicate updated plans to all stakeholders as they occur.
```

```card "Tenet 7: Continuous Collaboration" "580" "success"
Establishing continuous collaboration throughout the software development process means that teams should continuously collaborate with all stakeholders, including customers, product owners, developers, and testers.

Continuous collaboration helps to ensure that everyone is aligned with the project goals and objectives, and that everyone has a shared understanding of the product requirements and features.

#### Foster a Collaborative Culture

Establish a collaborative culture that encourages open communication, sharing of ideas, and respect for all team members.

#### Collaborate Throughout the Development Process

Collaborate with all stakeholders throughout the
development process, from requirements gathering to release planning.

#### Use Collaborative Tools and Processes

Use collaborative tools and processes, such as Notion and Teams, to facilitate collaboration among team members.

Take special note that this does not require every person to handle all communication themselves. Instead, it is generally ideal to utilize a hierarchy of contacts that will pass on communications to those that need it, minimizing any developer downtime while still allowing information to be properly communicated.

For example, a software engineer is making a change to the plan, so they communicate that change to the project manager. The project manager in turn will communicate that change to other engineers that are directly affected as well as key stakeholders. Those key stakeholders may then communicate that change to other stakeholders, and so on.
```

```card "Tenet 8: Continuous Learning" "580" "success"
Continuous learning means that developers and teams should be continuously learning to help improve their skills, processes, workflows, and products.

Continuous learning helps to ensure that teams stay up to date with the latest technologies, methodologies, and practices, and that they continuously improve the quality of their products and the efficiency of their development process.

#### Emphasize Continuous Learning

Prioritize learning throughout the development process, from initial requirements gathering to post-release evaluations.

#### Encourage Experimentation

Encourage team members to experiment with new ideas, technologies, methodologies, and practices to continuously improve the development process and products.

#### Foster a Culture of Learning

Foster a culture of learning that encourages sharing of knowledge and experiences, both positive and negative, among team members and provides opportunities for learning and growth.
```

```card "Tenet 9: Continuous Testing" "580" "success"
Establishing continuous testing throughout the software development process means that teams should continuously test their products throughout the development process, from requirements gathering to release, utilizing automated testing as much as possible.

Continuous testing helps to ensure that teams catch and fix defects as early as possible in the development process, which then helps to reduce time resolving those defects.

#### Establish a Test-Driven Development (TDD) approach

Apply a TDD approach, where testing is integrated into the development process, and tests are written before the code is developed.

#### Automate Testing

Automate testing as much as possible, to increase efficiency and consistency. If a tool isn't available to apply the type of testing you need, then take the time to build the tool so you can apply your tests.

#### Test Continuously

Test continuously throughout the development process, including unit tests, integration tests, and end-to-end tests.
```

```card "Tenet 10: Continuous Iterations" "580" "success"
Establishing continuous iterations throughout the software development process means that teams should continuously iterate on their products and processes to improve their quality and efficiency.

Continuous iterations help to ensure that teams are always improving their products and processes, and that they are continuously adapting to changing requirements and market needs.

#### Emphasize Continuous Improvement

Prioritize continuous improvement throughout the development process, from initial requirements gathering to post-release evaluations.

#### Conduct Regular Retrospectives

Conduct regular retrospectives to review the development process and identify areas for improvement.

#### Adapt Quickly

Adapt quickly to changing requirements and market needs by iterating on products and processes.
```

```card "Tenet 11: Continuous Integrations" "580" "success"
Establishing continuous integrations throughout the software development process means that teams should continuously integrate their code changes into a single source of truth, such as a shared repository while only using a single main branch and test their changes as frequently as possible.

Continuous integrations help to ensure that teams are always working with the latest version of the codebase and that any issues are identified and resolved as quickly as possible.

#### Single Branch Version Control

Use a version control system to manage code changes and ensure that all team members are working with the latest version of the codebase. Ensure all development is done directly in a single main branch.

#### Automate Integration Testing

Automate integration testing to ensure that code changes are tested thoroughly and that any issues are identified and resolved as quickly as possible.

#### Integrate Frequently

Integrate code changes into the shared repository as frequently as possible, to ensure that team members are always working with the latest version of the codebase.
```

```card "Tenet 12: Continuous Delivery" "580" "success"
Establishing continuous delivery throughout the software development process means that teams should aim to deploy their code changes to production as frequently as possible through completely automated processes.

Continuous delivery helps to ensure that teams can deliver new features and improvements to their users quickly and efficiently. It also helps to reduce the risk associated with deploying code changes to production by automating the deployment process and ensuring that code changes have been thoroughly tested before deployment.

#### Automate Production Deployment

Automate the deployment process from the moment code is pushed up from the developer's machine all the way through production, only stopping from deploying if an automated test fails.

#### Automate Development Deployments

Keep tooling, documentation, and scripts available and easily accessible so any developer working on a project can quickly and easily setup local development environments with a focus on making the process as self-service as possible.

#### Monitor and Measure

Monitor and measure the performance of the software after every deployment to assure quality standards are met and performance never degrades unexpectedly. Again, this should be largely automated, but still apply manual checks as needed.
```
````

`````

```section
To help further expand on the core tenets and principles defined in CASE, here are some additional best practices and workflows that I have found to be the most productive for delivering the best quality software being delivered to customers, the most maintainable software for developers to iterate and deliver new features quickly, and the best user experiences, all while simultaneously delivering at a much faster pace than any team I've seen using any type of Scrum-based workflows.
```

`````cards

````card "Never Backlog Bugs" "700" "primary"
Anyone involved with the development of a project should include testing for bugs as a normal part of their day-to-day workflow. And when bugs are encountered document them enough that they can be reproduced easily by a developer.

As soon as a bug is documented, it should be a top priority to fix. Do not backlog bugs.

When researching and resolving bugs, use this time to add any additional tests that are triggered to fail by this bug. This will provide a verifiable metric to show the bug is resolved.

Most developers and teams that struggle with this are struggling because they don't implement good workflows and testing strategies.

```quote "success"
Getting rid of bugs as early as possible means they don't have time to grow and pollute other parts of your systems. This provides the long-term benefit of greatly reducing time spent on bugs, as well as the benefit of your users getting to enjoy a better experience that's not affected by bugs.
```
````

````card "Limit Scheduled / Ongoing Meetings" "700" "primary"
Admin, managers, etc. should not constrain developers by frequent meetings or events. Ideally, a developer would have no more than 1 or 2 hours of dedicated meetings in a week.

Larger companies that absolutely must have a lot of meetings should focus on restricting mandatory meetings to no more than 1 day per week.

It is generally good to have a weekly meeting day set for developers. If a developer is needed in a meeting, set that meeting on the meeting day.

This restrainment does not include developer organized meetings, as developers will want and need to meet with others as part of their development practices as they see is needed or desired. But these are often relatively rare occurrances, as most developers in my experience prefer exhanging information through email or chat when possible.

```quote "success"
More time is given to focus on productive development.

Developers have fewer interruptions and breaks to their thought processing, which means less time wasted stopping to prep for a meeting, then taking time to get the mind adjusted back to the current task.
```
````

````card "Trust Developers to Research/Test/Decide on solutions" "700" "primary"
```quote "danger"
One of the more prominent and dangerous practices in modern teams is this concept of discussing and finding a solution as a team, usually voting on a solution within a team meeting to get a "concrete" direction for a developer or project to take, without any research being done to properly evaluate the problem, nor any researching and prototyping of potential solutions. Technical decisions are in effect being driven by feelings and preferences.

Team-based decision-making diminishes or often removes any expectation to take time to properly analyze and research the problem, theorize possible solutions, prototype and test various potential solutions, and finalize a decision based on provable metrics.
```

A developer should own a feature/problem, research a few proposals with pros and cons, then bring it up to the team for discussion only if they can't make an absolute decision based on data-driven results.

Team discussions are still encouraged if the owning developer wants different perspectives, especially if others on the team are known to be experts in the area of the problem, but any direction from other developers should be taken as suggestions or advice for a place to start, not an absolute way of doing something.

```quote "success"
A developer can make more decisive decisions based on their expertise, research, and testing. Meaning more data-driven decisions and fewer feelings-driven decisions.

Confidence and trust builds for everyone within the team as everyone is making educated, data-driven decisions to solve problems, instead of virtually all solutions coming from 1 or 2 developers who happen to be the most vocal, pushy, or otherwise dictated as a decision maker.
```
````

````card "No Scrum / No Sprints / No Time or Point estimates" "700" "primary"
```quote "danger"
This concept is included in the tenets `Continuous Agility` and `Continuous Planning`, but I am explicitly pointing it out here for brevity.

In every Scrum team that I have been a part of, I've noticed that developers often spend more time working on planning, documenting work to be done, and calculating estimates before starting any actual development than it would take to just develop the solution.

Attaching estimates or deadlines to work to be done has a cascading effect that reduces code quality and maintainability, increases bugs, reduces feature quality, reduces product quality and customer experiences, and ultimately causes new features and updates to take longer as technical debt increases and updates are harder to implement.
```

Focus on the present instead of wasting time trying to predict the future. No time or pointing estimates in any way, or documenting every step of work to be done ahead of time.

Set "goals" instead of deadlines. Product owners should set and prioritize goals as desired, but do not set or ask for any deadlines for any goal. Developers should also set their own goals and prioritize their development goals within the product owner's goals.

```quote "warning"
If you must have estimates, then limit long-term estimates to managers providing estimates on features, and developers only providing estimates for tasks as they start the task. Additionally, do not spend more than a few seconds coming up with an estimate. Estimates are guesses, and more time spent working on estimates means less time working on more productive work.
```

```quote "success"
Developers are not set up for failure with pressure to finish a task with a deadline that doesn't give proper time to do proper research and testing.

Time wasted ramping up new sprints and closing previous sprints is gone. Developers can jump from one task to the next without needing to wait for the next sprint because there isn't enough time in the current sprint to finish the new task.
```
````

`````

```paper "ml-a mr-a mt-5"
Visit SoftwareStandards.dev for more [detailed information on engineering software with agile and continuous delivery systems](https://www.softwarestandards.dev).
```
