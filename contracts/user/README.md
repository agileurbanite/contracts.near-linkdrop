Set env variable (example for git bash)
`export CONTRACT_ID="dev-1623415128942-51140410425140"`

Init a campaign contract
`near call $CONTRACT_ID new '{}' --account-id $CONTRACT_ID`

leaf shop source fish rally length trial measure wise sponsor draft shadow Pk:
4hXSafecZtcbiJH5WQAJPBxfsTPEv3kJYkoctJrDtNLN Sk:
4rXFz4HRThnTaXGmC7NuHSKbpdMf2k4um1ZgEhGT97myX5TWaLPDkAb6QBNV2MRtPzhXeH1USWqruDugeq6W8xEz

`near call $CONTRACT_ID create_near_campaign '{"name":"campaign1","public_key":"4hXSafecZtcbiJH5WQAJPBxfsTPEv3kJYkoctJrDtNLN", "tokens_per_key":"500000000000000000000000"}' --account-id $CONTRACT_ID --amount 5 --gas 300000000000000`

Get Campaigns
`near view $CONTRACT_ID get_campaigns '{}'`