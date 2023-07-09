# Notes

# BofA

Need to cut off the top half of the csv file

# Chase Bank

For whatever reason the first line doesn't match up, so add a `,` right before the `#`
```
Details,Posting Date,Description,Amount,Type,Balance,Check or Slip,#
DEBIT,07/03/2023,"Online Transfer to SAV ...8537 transaction#: 17785897409 07/03",-1939.42,ACCT_XFER,0.00,,
```

# Venmo

So many issues
1) Some of the csv records span multiple lines, so that needs to manually be fixed
2) The last line does not get parsed correctly since its multiline, so delete
3) Remove the header lines that denote beginning balance, etc...

# Bilt

Get the auth bearer token from webpage network inspector

Run to get json
```
curl --request GET \
  --url 'https://api.biltrewards.com/wf/transactions?startDateTime=2023-01-01T04%3A00%3A01Z&endDateTime=2023-07-01T03%3A59%3A59Z' \
  --header 'Authorization: Bearer <TOKEN>
```

Convert JSON to CSV