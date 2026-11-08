<webui-data data-page-title="Software Engineering Standards & Practices" data-page-subtitle="The twelve tenets of CASE"></webui-data>
<webui-data data-page-next-page='{"name":"CASE Book","href":"/blogs/book_published_for_case_continuous_agile_software_engineering"}'></webui-data>

<webui-page-segment elevation="10">

Continuous Agile Software Engineering, or CASE, is the set of standards and practices I've developed and refined over my career for building quality software in small, tested steps and delivering it continuously. This page is a brief introduction. My book, [CASE: Continuous Agile Software Engineering](/blogs/book_published_for_case_continuous_agile_software_engineering), covers it in full, and [SoftwareStandards.dev](https://www.softwarestandards.dev) documents the standards in more depth.

I was able to use all of CASE at General Services Corp from 2016 to 2018, where I led greenfield projects from inception through delivery and maintenance. There I built a reporting portal that went live in its first week and grew to hundreds of reports, while pushing to production several times a day.

Many of the tenets are useful on their own, but CASE works best when they are adopted together. It also works best where management trusts engineers to own their work without micromanagement, and engineers are motivated to do it well.

</webui-page-segment>

## Twelve Tenets of Continuous Agile Software Engineering

<webui-page-segment elevation="10">
    The CASE methodologies are broken up into twelve tenets, with each tenet containing three core principles.
</webui-page-segment>
<webui-page-segment elevation="10">
    <webui-cards card-width="580">
        <webui-card name="Tenet 1: Keep it Simple" theme="success">
            Keeping things simple means that software development should focus on delivering value in the simplest viable way. This is achieved by minimizing complexity in code, processes, and tools.
            Keeping things simple needs to be both a mentality and a habit, and is aimed at helping to reduce the cost of development, improve quality of both the code and software, and reduce the time to market. It also helps to keep the development process agile, as simpler code and processes are easier to change and adapt to new requirements.
            #### Minimize Complexity
            Avoid unnecessary complexity in code, processes, and tools. This helps to reduce the cost of development, improve quality throughout, and reduce the time to market.
            #### Eliminate Waste
            Eliminate waste in the development process by removing unnecessary tasks, documentation, and other activities that do not add value. Eliminate code that is not being used and code comments that are not adding value.
            #### Prioritize Value
            Prioritize value over other factors such as features, aesthetics, and performance. This helps to ensure that development efforts are focused on delivering the most key features first. But also understand value doesn't have to be monetarily based or exclusive to customers / users. Some value may be for developers, stakeholders, or others as well.
        </webui-card>
        <webui-card name="Tenet 2: Code Ownership" theme="success">
            Code Ownership is the concept that each repository within a team should have a designated owner responsible for its maintenance and improvement over time. The following are three core principles that can guide the implementation of Code Ownership:
            #### Assign Ownership of Code
            Each code repository should have a designated owner responsible for its overall quality, maintenance, and improvement over time. This owner should be the primary point of contact for all matters related to the code within that repository and have the authority to make decisions about its evolution, including code formatting and organization.
            #### Foster Collaboration Among Owners
            Ownership does not mean isolation. Code owners should collaborate with each other to share ideas and concepts and learn from each other's different perspectives and opinions. They should communicate frequently and openly to avoid redundancy, minimize conflicts, and make sure everyone is aligned with the business and team goals.
            #### Rotate Ownership Over Time
            Code ownership is not a lifelong appointment. As projects evolve, so do the skills, interests, and availability of team members. To ensure that code remains fresh and up-to-date, and owners do not grow stagnant, ownership should rotate periodically. This approach fosters a sense of shared responsibility and prevents bottlenecks that can occur when a single person is responsible for too much code.
        </webui-card>
        <webui-card name="Tenet 3: Continuous Agility" theme="success">
            Continuous Agility means that teams should continuously adapt to changing requirements and market needs and be able to pivot quickly when necessary.
            Continuous agility helps to ensure that teams can respond to new challenges and opportunities, and that they are always focused on delivering value to their customers.
            #### Never follow with Absolution
            When working within principles, guidelines, best practices, never constrain yourself to following them with absolute adherence. There is no one-size-fits-all solution that is the best solution for every situation.
            #### Be Customer-Focused
            Maintain a customer-focused mindset and continuously seek feedback to ensure that products are meeting their needs and expectations and adapt tasks based on the feedback loops.
            #### No Estimates or Time-Boxing
            Break down projects and features into manageable tasks, but no task, feature, or project should be given deadlines or require developer estimates prior to or during active development.
        </webui-card>
        <webui-card name="Tenet 4: Continuous Feedback Loops" theme="success">
            Establishing continuous feedback loops throughout the software development process means that developers and teams should continuously gather feedback from their IDEs, compilers, analytics, stakeholders, customers, and team members, and use that feedback to refine the development process and features being delivered.
            Continuous feedback loops help teams and developers to stay focused on delivering value to customers, and to identify and address issues as early as possible in the development process. This helps to reduce the cost of development, improve quality, and increase customer satisfaction.
            #### Gather Feedback
            Continuously gather feedback from their tools, analytics, stakeholders, customers, and team members throughout the development process.
            #### Use Feedback to Inform Decisions
            Use feedback to inform decisions about features, design, and other aspects of the development process.
            #### Iterate Based on Feedback
            Continuously iterate on the development process based on feedback to improve the product and development process.
        </webui-card>
        <webui-card name="Tenet 5: Continuous Automation" theme="success">
            Establishing continuous automation throughout the software development process means that teams should continuously automate repetitive and time-consuming tasks, such as testing, building, and deployment.
            Continuous automation helps to reduce the time and effort required for these tasks, which in turn helps to increase productivity, improve quality, and reduce the time to market.
            #### Automate Everything Possible
            Identify and automate every task that can be automated, such as testing, building, and deployment.
            #### Integrate Automation into the Development Process
            Integrate automation tools and processes into the development process to reduce manual effort and increase efficiency. Make this a top priority, and do it as early as possible.
            #### Continuously Improve Automation
            Continuously improve automation processes to ensure that they are reliable, efficient, and effective.
        </webui-card>
        <webui-card name="Tenet 6: Continuous Planning" theme="success">
            Establishing continuous planning throughout the software development process means that teams should continuously plan and adjust the development process based on feedback and changing requirements. This also means that teams should spend minimal time planning development or features that are not being immediately worked on.
            Continuous planning helps teams to stay aligned with customer needs and market trends, and to make timely adjustments to the development process as needed. This helps to reduce the risk of developing products that do not meet customer needs or fail in the market.
            #### Emphasize Continuous Planning
            Prioritize planning throughout the development process, from initial requirements gathering to release planning.
            #### Use Data-Driven Approaches
            Use data-driven approaches, such as user feedback, A-B testing, and market trends, to inform planning decisions and adjust the development process.
            #### Continuously Refine Plans
            Continuously refine plans based on feedback, discoveries made during development, and changing requirements, and communicate updated plans to all stakeholders as they occur.
        </webui-card>
        <webui-card name="Tenet 7: Continuous Collaboration" theme="success">
            Establishing continuous collaboration throughout the software development process means that teams should continuously collaborate with all stakeholders, including customers, product owners, developers, and testers.
            Continuous collaboration helps to ensure that everyone is aligned with the project goals and objectives, and that everyone has a shared understanding of the product requirements and features.
            #### Foster a Collaborative Culture
            Establish a collaborative culture that encourages open communication, sharing of ideas, and respect for all team members.
            #### Collaborate Throughout the Development Process
            Collaborate with all stakeholders throughout the development process, from requirements gathering to release planning.
            #### Use Collaborative Tools and Processes
            Use collaborative tools and processes, such as Notion and Teams, to facilitate collaboration among team members.
            Take special note that this does not require every person to handle all communication themselves. Instead, it is generally ideal to utilize a hierarchy of contacts that will pass on communications to those that need it, minimizing any developer downtime while still allowing information to be properly communicated.
            For example, a software engineer is making a change to the plan, so they communicate that change to the project manager. The project manager in turn will communicate that change to other engineers that are directly affected as well as key stakeholders. Those key stakeholders may then communicate that change to other stakeholders, and so on.
        </webui-card>
        <webui-card name="Tenet 8: Continuous Learning" theme="success">
            Continuous learning means that developers and teams should be continuously learning to help improve their skills, processes, workflows, and products.
            Continuous learning helps to ensure that teams stay up to date with the latest technologies, methodologies, and practices, and that they continuously improve the quality of their products and the efficiency of their development process.
            #### Emphasize Continuous Learning
            Prioritize learning throughout the development process, from initial requirements gathering to post-release evaluations.
            #### Encourage Experimentation
            Encourage team members to experiment with new ideas, technologies, methodologies, and practices to continuously improve the development process and products.
            #### Foster a Culture of Learning
            Foster a culture of learning that encourages sharing of knowledge and experiences, both positive and negative, among team members and provides opportunities for learning and growth.
        </webui-card>
        <webui-card name="Tenet 9: Continuous Testing" theme="success">
            Establishing continuous testing throughout the software development process means that teams should continuously test their products throughout the development process, from requirements gathering to release, utilizing automated testing as much as possible.
            Continuous testing helps to ensure that teams catch and fix defects as early as possible in the development process, which then helps to reduce time resolving those defects.
            #### Establish a Test-Driven Development (TDD) approach
            Apply a TDD approach, where testing is integrated into the development process, and tests are written before the code is developed.
            #### Automate Testing
            Automate testing as much as possible, to increase efficiency and consistency. If a tool isn't available to apply the type of testing you need, then take the time to build the tool so you can apply your tests.
            #### Test Continuously
            Test continuously throughout the development process, including unit tests, integration tests, and end-to-end tests.
        </webui-card>
        <webui-card name="Tenet 10: Continuous Iterations" theme="success">
            Establishing continuous iterations throughout the software development process means that teams should continuously iterate on their products and processes to improve their quality and efficiency.
            Continuous iterations help to ensure that teams are always improving their products and processes, and that they are continuously adapting to changing requirements and market needs.
            #### Emphasize Continuous Improvement
            Prioritize continuous improvement throughout the development process, from initial requirements gathering to post-release evaluations.
            #### Conduct Regular Retrospectives
            Conduct regular retrospectives to review the development process and identify areas for improvement.
            #### Adapt Quickly
            Adapt quickly to changing requirements and market needs by iterating on products and processes.
        </webui-card>
        <webui-card name="Tenet 11: Continuous Integrations" theme="success">
            Establishing continuous integrations throughout the software development process means that teams should continuously integrate their code changes into a single source of truth, such as a shared repository while only using a single main branch and test their changes as frequently as possible.
            Continuous integrations help to ensure that teams are always working with the latest version of the codebase and that any issues are identified and resolved as quickly as possible.
            #### Single Branch Version Control
            Use a version control system to manage code changes and ensure that all team members are working with the latest version of the codebase. Ensure all development is done directly in a single main branch.
            #### Automate Integration Testing
            Automate integration testing to ensure that code changes are tested thoroughly and that any issues are identified and resolved as quickly as possible.
            #### Integrate Frequently
            Integrate code changes into the shared repository as frequently as possible, to ensure that team members are always working with the latest version of the codebase.
        </webui-card>
        <webui-card name="Tenet 12: Continuous Delivery" theme="success">
            Establishing continuous delivery throughout the software development process means that teams should aim to deploy their code changes to production as frequently as possible through completely automated processes.
            Continuous delivery helps to ensure that teams can deliver new features and improvements to their users quickly and efficiently. It also helps to reduce the risk associated with deploying code changes to production by automating the deployment process and ensuring that code changes have been thoroughly tested before deployment.
            #### Automate Production Deployment
            Automate the deployment process from the moment code is pushed up from the developer's machine all the way through production, only stopping from deploying if an automated test fails.
            #### Automate Development Deployments
            Keep tooling, documentation, and scripts available and easily accessible so any developer working on a project can quickly and easily setup local development environments with a focus on making the process as self-service as possible.
            #### Monitor and Measure
            Monitor and measure the performance of the software after every deployment to assure quality standards are met and performance never degrades unexpectedly. Again, this should be largely automated, but still apply manual checks as needed.
        </webui-card>
    </webui-cards>
</webui-page-segment>
<webui-page-segment elevation="10">
    Beyond the tenets, these are practices I've found most effective for delivering quality software quickly, keeping code easy to change, and giving users the best experience.
</webui-page-segment>
<webui-cards card-width="700">
    <webui-card name="Never Backlog Bugs" theme="primary">
        Everyone involved in a project should test for bugs as part of their normal work, and document any bug well enough that a developer can reproduce it easily.
        As soon as a bug is documented, make fixing it a top priority. Do not put bugs on a backlog.
        While fixing a bug, add tests that fail because of it. Those tests prove the bug is fixed and keep it from coming back.
        <webui-quote theme="success">
            Fixing bugs early means they don't have time to grow and spread into other parts of the system. Over time, the team spends far less time on bugs, and users get a better experience.
        </webui-quote>
    </webui-card>
    <webui-card name="Protect Focus Time" theme="primary">
        Keep scheduled meetings for developers to a minimum. Ideally, a developer has no more than one or two hours of recurring meetings a week.
        Organizations that need more meetings can set one meeting day a week for developers, and schedule anything that needs a developer on that day.
        This doesn't apply to meetings developers arrange themselves when they need to work through something with others, though most developers prefer email or chat when they can use it.
        <webui-quote theme="success">
            Developers get more uninterrupted time for focused work, and lose less time preparing for meetings and getting back into a task afterward.
        </webui-quote>
    </webui-card>
    <webui-card name="Trust Developers to Research, Test, and Decide" theme="primary">
        A developer should own a feature or problem, research a few options with their pros and cons, and test them. They bring the decision to the team only when the data doesn't settle it.
        Team discussions are still welcome when the owner wants other perspectives, especially from teammates who are experts in the area. That input is advice and a place to start, not a mandate.
        <webui-quote theme="warning">
            Deciding on a solution by team discussion or vote, before anyone has researched the problem or tested possible solutions, tends to produce decisions based on preference rather than evidence.
        </webui-quote>
        <webui-quote theme="success">
            Decisions are based on research and measurable results. Trust grows across the team, because good solutions can come from anyone, not only the loudest voices.
        </webui-quote>
    </webui-card>
    <webui-card name="Goals Instead of Deadlines and Estimates" theme="primary">
        This practice comes from the Continuous Agility and Continuous Planning tenets.
        Focus on the work in front of you rather than trying to predict the future. Product owners set and prioritize goals, without deadlines, and developers set and prioritize their own goals within them. Avoid detailed time or point estimates, and avoid documenting every step of work in advance.
        <webui-quote theme="warning">
            If you must have estimates, keep long-term estimates at the feature level with managers, and have developers estimate a task only when they start it. Spend seconds on an estimate, not hours. Estimates are guesses, and time spent on them is time not spent building.
        </webui-quote>
        <webui-quote theme="success">
            Developers have the time to research and test properly instead of rushing to meet a deadline, and they move straight from one task to the next without waiting for a planning cycle.
        </webui-quote>
    </webui-card>
</webui-cards>
<webui-flex justify="center" class="my-5">
    Visit SoftwareStandards.dev for more [detailed information on engineering software with agile and continuous delivery](https://www.softwarestandards.dev).
</webui-flex>
