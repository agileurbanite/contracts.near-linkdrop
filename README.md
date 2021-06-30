#Near Linkdrop
Linkdrop is an application that allows you to send NEAR to other people via a link or
helps a new user to create the account - you can pay a gas fee instead of a user
and send NEAR to this account.

##Getting started


##Architecture
![High level architecture](docs/high-level-architecture.png)

NEAR Linkdrop - is a set of a few contracts.
We have 3 different types of contracts - Linkdrop, User and Campaign. Each contract is deployed to its
own account. We use name hierarchy here - every user account will be a subaccount of `linkdrop.testnet`
(e.g `bob.linkdrop.testnet`), and each campaign account will be a subaccount of the user account
(e.g `my-first-campaign.bob.linkdrop.testnet`).

Linkdrop - it's a root contract of the app and the entry point for a new users. This contract 
creates a new user account and deploys a user contract code to it.

User - this is a contract that allows a user to create new campaigns. This account is bounded with the
user Wallet account, which means if the Wallet account id is `bob.testnet` his Linkdrop account id
will be `bob.linkdrop.testnet`

Campaign - it's a contract that an end user will interact with (using a link). Right now we have
only one type of the campaign - NEAR campaign, but it will possible to add new campaigns type such as 
NFT, NFT-FT, NFT-NEAR etc.


