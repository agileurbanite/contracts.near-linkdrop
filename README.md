# Near Linkdrop V1
Linkdrop is an application that allows you to send NEAR to other people via a link or
helps a new user to create the account - you can pay a gas fee instead of a user
and send NEAR to this account.

## Architecture
![High level architecture](docs/high-level-architecture.png)

NEAR Linkdrop - is a set of a few contracts which are designed to use together - you can't use a Near Campaign contract
without a User contract.

We have 3 different types of contracts - Linkdrop, User and Near Campaign. Each contract is deployed to its
own account. We use name hierarchy here - every user account will be a subaccount of `linkdrop.testnet`
(e.g `bob.linkdrop.testnet`), and each campaign account will be a subaccount of the user account
(e.g `my-first-campaign.bob.linkdrop.testnet`). Linkdrop account is a factory for User accounts and User is a factory 
for a Near Campaign accounts.

Linkdrop - it's a root contract of the app and a factory of user accounts. This contract 
creates a new user account and deploys a user contract code to it.

User - this is a contract that allows a user to create new campaigns. This account is bounded with the
user Wallet account, which means if the Wallet account id is `bob.testnet` his Linkdrop account id
will be `bob.linkdrop.testnet`

Campaign - it's a contract that an end user will interact with (using a link). Right now we have
only one type of the campaign - NEAR campaign, but it will be possible to add new campaigns type such as 
NFT, NFT-FT, NFT-NEAR etc.

## NEAR campaign
Near campaign - a contract that allows you to create the set of links. Each link will be unique and will
contain some amount of NEAR, which is the same for all links of the campaign. There are two ways how end-user can use 
this link - create a new account or get NEAR on his existing account.
For both of these actions we use the 'root account' - `testnet` in the testnet network and `near` in the mainnet.

## How to use via NEAR CLI
1. First of all, we need to deploy Linkdrop contract to testnet:\
`near dev-deploy wasm/linkdrop.wasm`\
It will create a dev account like `dev-1633605027594-34986547928831` and initialize it.
####
2. Set LINKDROP_ID in a terminal (Ubuntu):\
`export LINKDROP_ID="dev-1633605027594-34986547928831"`
####
3. Generate a new local access key for a future user account:\
`near generate-key alice.dev-1633605027594-34986547928831`\
It will return a public key like `58c1dJaULjQys3XgPRyW49aKAndMaWEcLBwyCpzSHVcT`
####
4. Let's create a new linkdrop user account:\
`near call $LINKDROP_ID create_user_account 
'{ "name":"alice",
"public_key":"58c1dJaULjQys3XgPRyW49aKAndMaWEcLBwyCpzSHVcT"}' 
--accountId $LINKDROP_ID --amount 20 --gas 300000000000000`
####
5. Set USER_ID:
`export USER_ID="alice.dev-1633605027594-34986547928831"`
####
6. Now we have a user account - `alice.dev-1633603526196-33735380823540` which is ready to use. We 
can interact with it is the next ways:
   * Get current version of the user contract:\
   `near view $USER_ID get_user_metadata '{}'` -> `{ version: '1.0' }`
   * Get list of user campaigns:\
   `near view $USER_ID get_campaigns '{}'` -> `[]`
   * Create a new campaign
####
7. Generate a new key for a campaign:\
`near generate-key campaign.alice.dev-1633605027594-34986547928831`\
It will return a public key like `5zTGgvfEWcdUfwiuizhNSLkXd7MdvuPqG3CGHAaXsjNV`
####
8. Create a new campaign with 2 links and each one will contain 1 NEAR:
`near call $USER_ID create_near_campaign 
'{"name":"campaign",
"public_key":"5zTGgvfEWcdUfwiuizhNSLkXd7MdvuPqG3CGHAaXsjNV", 
"total_keys": 2, 
"tokens_per_key": "100000000000000000000000", 
"account_creator": "testnet"}' 
--accountId $USER_ID --amount 10 --gas 300000000000000`
####
9. Set NEAR_CAMPAIGN_ID:
   `export NEAR_CAMPAIGN_ID="campaign.alice.dev-1633605027594-34986547928831"`
####
9. We created a campaign, but it is not ready to use - we need to add keys. In this example we will
use the next 2 access keys:
    * pk1: `8bFrYwXUEvLH5zkzGn2fG2bKjJu3kNNP4xXqsBvc2nJe`\
      sk1: `39qnXSsiUUtuyMMJBkepa3qfv44qe6ZfixEMC9no1v6kjnaaKYj1pZ8pFmci1rSE9c2GsMVhF2NpXgu5aAYbCq3Y`
    * pk2: `HYE4EaDGTFJvDFA7BD8GpkHbsWga5KB48xUckhuSC7mz`\
      sk2: `7B7NKGq3GeHgBubRaCFiU4tBaYwJsq3LBKYZttEonHnnupJJMYLSmgRkEc8hJWemH8YxgDJKbWs7qcu8UfuZFck`

    Let's add these keys:\
    `near call $NEAR_CAMPAIGN_ID add_keys '{"keys": 
    ["8bFrYwXUEvLH5zkzGn2fG2bKjJu3kNNP4xXqsBvc2nJe", 
    "HYE4EaDGTFJvDFA7BD8GpkHbsWga5KB48xUckhuSC7mz"]}'
    --accountId $NEAR_CAMPAIGN_ID --gas 300000000000000`

####
10. Now the campaign - `campaign.alice.dev-1633603526196-33735380823540` is ready to use. 
We can interact with it is the next ways:
    * Get campaign metadata:
    `near view $NEAR_CAMPAIGN_ID get_campaign_metadata '{}'`  -> \
    {\
      &nbsp;&nbsp;campaign_id: 1,\
      &nbsp;&nbsp;user_id: 'alice.dev-1633605027594-34986547928831',\
      &nbsp;&nbsp;tokens_per_key: '100000000000000000000000',\
      &nbsp;&nbsp;created_at: 1633611315680281900,\
      &nbsp;&nbsp;account_creator: 'testnet',\
      &nbsp;&nbsp;keys_stats: {\
      &nbsp;&nbsp;&nbsp;&nbsp;total: 2,\
      &nbsp;&nbsp;&nbsp;&nbsp;added_during_creation: 2,\
      &nbsp;&nbsp;&nbsp;&nbsp;deleted_during_deletion: 0,\
      &nbsp;&nbsp;&nbsp;&nbsp;active: 2,\
      &nbsp;&nbsp;&nbsp;&nbsp;created: 0,\
      &nbsp;&nbsp;&nbsp;&nbsp;claimed: 0,\
      &nbsp;&nbsp;&nbsp;&nbsp;refunded: 0\
      &nbsp;&nbsp;},\
      &nbsp;&nbsp;status: 'Active',\
      &nbsp;&nbsp;version: '1.0'\
      }
    * Get keys status:\
    `near view $NEAR_CAMPAIGN_ID get_keys '{"keys":
      ["8bFrYwXUEvLH5zkzGn2fG2bKjJu3kNNP4xXqsBvc2nJe",
      "HYE4EaDGTFJvDFA7BD8GpkHbsWga5KB48xUckhuSC7mz"]}'` -> \
      `[{
         pk: 'ed25519:8bFrYwXUEvLH5zkzGn2fG2bKjJu3kNNP4xXqsBvc2nJe',
         status: 'Active'
      }, {
         pk: 'ed25519:HYE4EaDGTFJvDFA7BD8GpkHbsWga5KB48xUckhuSC7mz',
         status: 'Active'
      }]`
    * Claim tokens by some another account or create a new account:\
      `https://wallet.testnet.near.org/linkdrop/campaign.alice.dev-1633603526196-33735380823540/
      39qnXSsiUUtuyMMJBkepa3qfv44qe6ZfixEMC9no1v6kjnaaKYj1pZ8pFmci1rSE9c2GsMVhF2NpXgu5aAYbCq3Y`
####
11. If we send the link to the wrong person you can delete it and refund you tokens:\
`near call $NEAR_CAMPAIGN_ID refund_keys '{
"keys":["8bFrYwXUEvLH5zkzGn2fG2bKjJu3kNNP4xXqsBvc2nJe"],
"beneficiary_id": "alice.dev-1633603526196-33735380823540"
}' --accountId $NEAR_CAMPAIGN_ID --gas 300000000000000`\
####
12. When campaign will be finished we can delete it and get the remains of NEAR back.
First, lets clear state - we can't delete the account with a large state (for example, 
if we will put 1000 keys): \
    `near call $NEAR_CAMPAIGN_ID clear_state '{
    "keys":["8bFrYwXUEvLH5zkzGn2fG2bKjJu3kNNP4xXqsBvc2nJe",
    "HYE4EaDGTFJvDFA7BD8GpkHbsWga5KB48xUckhuSC7mz"]
    }' --accountId $NEAR_CAMPAIGN_ID --gas 300000000000000` \
    Then we can delete the campaign itself: \
    `near call $NEAR_CAMPAIGN_ID delete_campaign 
    '{"beneficiary_id": "alice.dev-1633603526196-33735380823540"}' 
    --accountId $NEAR_CAMPAIGN_ID --gas 100000000000000`



    


