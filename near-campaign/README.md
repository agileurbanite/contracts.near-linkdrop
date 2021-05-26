Set env variable (example for git bash)
`export CONTRACT_ID="dev-1621937102125-94580031265469"`

Init a campaign contract
`near call $CONTRACT_ID new '{"tokens_per_key": "500000000000000000000000"}' --account-id $CONTRACT_ID`

Get Campaign State
`near view $CONTRACT_ID get_campaign '{}'`

Add Keys
`near call $CONTRACT_ID add_keys '{"keys": ["5GfoSNJfZWHK1RmLVKbiZ8xshsLiJESMMmqYxGyuiccK", "63QK9FHM83M4cZFbjVbFnotVj1Xz1pgWoLJfTX54m7Dm"]}' --account-id $CONTRACT_ID`

Create account or Claim Key
`https://wallet.testnet.near.org/create/dev-1621937102125-94580031265469/5a4B9cX8UeUaU9TrKqKkymUcK7At2hPpVBzATpMJL6bQfxgeLHBr9EjAzeRyS41izJ4wJMfYSBMQfAWuwuvkFQW7`

Delete Keys
`near call $CONTRACT_ID delete_keys '{"keys": ["63QK9FHM83M4cZFbjVbFnotVj1Xz1pgWoLJfTX54m7Dm"]}' --account-id $CONTRACT_ID`
