---
title: Questions and ideas
description: Questions and ideas about Kittynode.
---

This page is a collection of questions and ideas we have about Kittynode. We are building and thinking in the open.

## Questions

### How do we solve the "pump the gas coordination problem"?

This is a problem we noticed during the "pump the gas" initiative to increase the block gas limit on Ethereum. The general problem is that the Ethereum community cannot necessarily coordinate an increase in the block gas limit, even if a majority of the community votes for it. What we mean is that this is really gatekept by large institutional stakers (like Coinbase), as well as the core devs who decide on the default parameters in their clients. Orbit SSF should be able to democratize votes further, but this still feels like an issue. We should start by doing some more research and quantifying the problem.

### Should every internet connected device run an Ethereum node?

To have trustless access to the internet, it will be necessary unless they are bootstrapping with weak subjectivity endpoints. The current answer is yes, but it's unknown how many light clients vs stateless clients and we ought to do more research (there was a light clients talk at Devcon Thailand about this).

### What kind of node infrastructure do we support?

We expect operators to aggregate their validators, but potentially diversify and run multiple nodes for robustness. We do support these workflows, as it's simpler. However highly specialized use cases (like competitive block builders), is not something that makes sense for us to support due to the nature of it.

### Should voting in consensus require self-operation of a node, or can that part be delegated?

Current answer: We think that self-operation of a node is a must for maintaining self-sovereignty. There is no way to enforce these things in protocol, as it relates to the fork of the chain which is being run by the client.

### Do machines have the same voting rights as humans; and if so, does it open additional attack vectors?

Current answer: Seemingly yes, with some caveats. This has credible neutrality implications, which is always difficult to be completely credibly neutral. However most chains seek to be credibly neutral while minimizing bad things for all the users (like toxic MEV).

### Is there an overreliance on altruism?

It seems that there is some dependence on altruism for example in serving light clients, and also choosing to do home staking over LSTs, ETFs, etc. Some altruism seems to be important, and we have evidence of it working in societies. For example, there does exist altruistic volunteers in various domains. If we make the job of "picking up trash" really easy (make it really easy to help), then we can get more helpers.

### Orbit SSF questions

Why is 1 ETH the minimum? Can we do 0.01 ETH? Can we remove the minimum entirely?

### Restaking

How do we feel about restaking risks? Is there anything we can do to disincentivize [overloading Ethereum's consensus](https://vitalik.eth.limo/general/2023/05/21/dont_overload.html) aside from saying it? How big of an issue is it for our goals?

### The 100% ETH staked problem

This doesn't seem like an actual problem but will list it out anyways for completeness. [Near 100% ETH staked has economic risks](https://youtu.be/WTwZsCKFPao?feature=shared&t=433).

> You don't want the entire population of your country to be soldiers; you also don't want every ETH to be staked.

### Is running a node voting?

This is a bit of a loaded question, adding here for completeness. We should formalize an answer on this both in theory, but more importantly in practice.

## Ideas

### The internet stack of the future

1. Local WiFi routers and local modems no longer exist, and internet is provided via space satellites and terrestrial towers.
2. Internet is always on and you are always connected (99.999% or higher).
3. Data bandwidth could exceed 100 Gbps, and latency could be under 1 ms.
4. Centralized root name servers are replaced by decentralized DNS.

### The core dev problem

One problem in voting in networks is that you usually have to run a client that is from a group of client developers. One thing that concerns us is that it means a small group of people control the software that everyone is running.

Let's say the broad Ethereum community wants to enact a gas limit change. Let's assume that 95% of the community desires this change. The community decides that a great way to make this into a reality is to change the default parameters for the clients. However the community cannot create that change, even though they represent a majority. The clients are maintained on GitHub, and changes are approved by a smaller group of maintainers. While the community could fork the client, this fork would be unofficial, and will not gain security updates or new features from the canonical client maintainers.

One problem seems to be that 95% of the community is not 95% of the validators. If we can make the validator set more representative of the community, the community can more quickly enact change (there is still the [impressionable dumb user problem](#the-impressionable-dumb-user-problem)).

But if it were, then all those 95% of validators could easily enact the change through voting. If the minority of core devs diverge from the public opinion, they can refuse to support the popular fork. This causes a problem because the 95% of validators may lack the technical expertise to maintain the fork (bug patches, etc). However, if this majority would be able to secure some funds (to pay for gas or humans), maybe they could fund an initiative to maintain the fork.

### The impressionable dumb user problem

This is a classic human problem. The basic idea is that sometimes people don't know what's best for them. How does this play into consensus and voting? What if a large majority of dumb users can be manipulated into voting for something that they seemingly want but is against their interests? This is a valuable attack which can be enacted by an intelligent actor.

A philosopher king likely needs to ossify the important parts. Separation of power will likely help. We still need to research here because we'd think even the base rules should have an override mechanism by the users, even if difficult.

A futarchy is useful for educating the populace. Think about walking into a polling booth and you are able to view data on the candidates.

### Rough notes on governance

- Ethereum may need a constitution or manifesto.
- There may be a plutocracy problem in the validator set (see: [Governance, Part 2: Plutocracy Is Still Bad](https://vitalik.eth.limo/general/2018/03/28/plutocracy.html)).
- Ethereum may need an overarching identity (EF?) or treaty system for unification.
- Ethereum may be able to learn from (and currently resembles some) aspects of colonial democracies.
- Ethereum may need a new forum for constructive communication aside from X.
- Look into: House of Lords and House of Commons.

### Grand vision

Kittynode has a grand vision:

- The world should reflect the virtues of its people.

There are many work streams that support this vision:

1. Making it easier to be a validator by reducing the financial burden (eg. Orbit SSF).
2. Making it easier to be a validator by reducing the technical burden (eg. Kittynode).
3. Initiatives that make the validator set more representative of its users (eg. Kittynode).
4. Initiatives that safeguard features of the world that are in the best interest of its users (eg. ossification, constitution).
5. Initiatives that empower users to shape the world to the best of their ability (eg. futarchy, education).
6. Initiatives that give control over property to the property owners (eg. wallets).

Today, Kittynode is focused on delivering a control center for world computer operators.

However, we don't try to fit the current solution into the problem, so to speak. We always try to think of what the problem is, and what is the best way to solve it. We're willing to abandon any solution for another one that serves the grand vision better.
