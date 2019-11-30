use dns_stamp_parser::DnsStamp;


/// Test all DNS Stamp from the [list] by decode and encode and decode it again.
///
/// [list]: https://github.com/DNSCrypt/dnscrypt-resolvers/blob/master/v2/public-resolvers.md
#[test]
fn stamps() {
    // The list is from https://github.com/DNSCrypt/dnscrypt-resolvers/blob/master/v2/public-resolvers.md
    let stamps = ["sdns://AgcAAAAAAAAADTIxNy4xNjkuMjAuMjIgPhoaD2xT8-l6SS1XCEtbmAcFnuBXqxUFh2_YP9o9uDgNZG5zLmFhLm5ldC51awovZG5zLXF1ZXJ5",
        "sdns://AgMAAAAAAAAADjE2OC4yMzUuODEuMTY3ID4aGg9sU_PpekktVwhLW5gHBZ7gV6sVBYdv2D_aPbg4EmRucy1ueWMuYWFmbGFsby5tZQovZG5zLXF1ZXJ5",
        "sdns://AgMAAAAAAAAADjE2OC4yMzUuODEuMTY3ID4aGg9sU_PpekktVwhLW5gHBZ7gV6sVBYdv2D_aPbg4EmRucy1ueWMuYWFmbGFsby5tZQovZG5zLXF1ZXJ5",
        "sdns://AgMAAAAAAAAADjE3Ni41Ni4yMzYuMTc1ID4aGg9sU_PpekktVwhLW5gHBZ7gV6sVBYdv2D_aPbg4DmRucy5hYWZsYWxvLm1lCi9kbnMtcXVlcnk",
        "sdns://AQMAAAAAAAAAFDE3Ni4xMDMuMTMwLjEzMjo1NDQzILgxXdexS27jIKRw3C7Wsao5jMnlhvhdRUXWuMm1AFq6ITIuZG5zY3J5cHQuZmFtaWx5Lm5zMS5hZGd1YXJkLmNvbQ",
        "sdns://AQMAAAAAAAAAFDE3Ni4xMDMuMTMwLjEzMDo1NDQzINErR_JS3PLCu_iZEIbq95zkSV2LFsigxDIuUso_OQhzIjIuZG5zY3J5cHQuZGVmYXVsdC5uczEuYWRndWFyZC5jb20",
        "sdns://AgMAAAAAAAAADzE3Ni4xMDMuMTMwLjEzMiD5_zfwLmMstzhwJcB-V5CKPTcbfJXYzdA5DeIx7ZQ6EhZkbnMtZmFtaWx5LmFkZ3VhcmQuY29tCi9kbnMtcXVlcnk",
        "sdns://AgMAAAAAAAAADzE3Ni4xMDMuMTMwLjEzMCD5_zfwLmMstzhwJcB-V5CKPTcbfJXYzdA5DeIx7ZQ6Eg9kbnMuYWRndWFyZC5jb20KL2Rucy1xdWVyeQ",
        "sdns://AQMAAAAAAAAAGlsyYTAwOjVhNjA6OmJhZDI6MGZmXTo1NDQzIIwhF6nrwVfW-2QFbwrbwRxdg2c0c8RuJY2bL1fU7jUfITIuZG5zY3J5cHQuZmFtaWx5Lm5zMi5hZGd1YXJkLmNvbQ",
        "sdns://AQMAAAAAAAAAGVsyYTAwOjVhNjA6OmFkMjowZmZdOjU0NDMggdAC02pMpQxHO3R5ZQ_hLgKzIcthOFYqII5APf3FXpQiMi5kbnNjcnlwdC5kZWZhdWx0Lm5zMi5hZGd1YXJkLmNvbQ",
        "sdns://AQMAAAAAAAAAEzExNi4yMDMuNzAuMTU2OjQ0NDMgenKjVeH-LU7Opsyq1ljKZz14fHsngOK8OOeQ-cR2mAsjMi5kbnNjcnlwdC1jZXJ0LmRuc3dhcmRlbi0xLWFkYmxvY2s",
        "sdns://AQMAAAAAAAAAHFsyYTAxOjRmODoxYzFjOjc1YjQ6OjFdOjQ0NDMgenKjVeH-LU7Opsyq1ljKZz14fHsngOK8OOeQ-cR2mAsjMi5kbnNjcnlwdC1jZXJ0LmRuc3dhcmRlbi0xLWFkYmxvY2s",
        "sdns://AQMAAAAAAAAAEzExNi4yMDMuMzUuMjU1OjQ0NDMg-IlTTFFgMuUntnNV78COzbhJN9_OEOVWNgHhdg4BNXwjMi5kbnNjcnlwdC1jZXJ0LmRuc3dhcmRlbi0yLWFkYmxvY2s",
        "sdns://AQMAAAAAAAAAHFsyYTAxOjRmODoxYzFjOjVlNzc6OjFdOjQ0NDMg-IlTTFFgMuUntnNV78COzbhJN9_OEOVWNgHhdg4BNXwjMi5kbnNjcnlwdC1jZXJ0LmRuc3dhcmRlbi0yLWFkYmxvY2s",
        "sdns://AgMAAAAAAAAADjExNi4yMDMuNzAuMTU2ID4aGg9sU_PpekktVwhLW5gHBZ7gV6sVBYdv2D_aPbg4EWRvaC5kbnN3YXJkZW4uY29tCC9hZGJsb2Nr",
        "sdns://AgMAAAAAAAAAF1syYTAxOjRmODoxYzFjOjc1YjQ6OjFdID4aGg9sU_PpekktVwhLW5gHBZ7gV6sVBYdv2D_aPbg4EWRvaC5kbnN3YXJkZW4uY29tCC9hZGJsb2Nr",
        "sdns://AgMAAAAAAAAADjExNi4yMDMuMzUuMjU1ID4aGg9sU_PpekktVwhLW5gHBZ7gV6sVBYdv2D_aPbg4EWRvaC5kbnN3YXJkZW4uY29tCC9hZGJsb2Nr",
        "sdns://AgMAAAAAAAAAF1syYTAxOjRmODoxYzFjOjVlNzc6OjFdID4aGg9sU_PpekktVwhLW5gHBZ7gV6sVBYdv2D_aPbg4EWRvaC5kbnN3YXJkZW4uY29tCC9hZGJsb2Nr",
        "sdns://AgMAAAAAAAAADjE0Ni4xODUuMTY3LjQzID4aGg9sU_PpekktVwhLW5gHBZ7gV6sVBYdv2D_aPbg4FGFkcy1kb2guc2VjdXJlZG5zLmV1Ci9kbnMtcXVlcnk",
        "sdns://AgMAAAAAAAAAHFsyYTAzOmIwYzA6MDoxMDEwOjplOWE6MzAwMV0gPhoaD2xT8-l6SS1XCEtbmAcFnuBXqxUFh2_YP9o9uDgUYWRzLWRvaC5zZWN1cmVkbnMuZXUKL2Rucy1xdWVyeQ",
        "sdns://AgcAAAAAAAAAAKA-GhoPbFPz6XpJLVcIS1uYBwWe4FerFQWHb9g_2j24OKBoo-sB-l8CxNNfOhHQBMrwiyJL7qfXnFiMfxPIYTNgLqDvR4Wu5wydV1_nM4MG2T6nlhHl_tzvU2LdZsmLYLstvSAcVDa2UaK1QVwWz9ltGpcJ_ZyPJ-73XPlz2YL_5o5Y8BZkb2guYXBwbGllZHByaXZhY3kubmV0Bi9xdWVyeQ",
        "sdns://AgUAAAAAAAAAACA-GhoPbFPz6XpJLVcIS1uYBwWe4FerFQWHb9g_2j24OAZkb2gubGkKL2Rucy1xdWVyeQ",
        "sdns://AgMAAAAAAAAADDEwNC4yOC4yOC4zNCBPtWwTIp4-T40ZbjCdyCfeStS1-WkKW8w_WWEQubJpyQ5kb2gudGlhcmFwLm9yZwovZG5zLXF1ZXJ5",
        "sdns://AgMAAAAAAAAAGVsyNjA2OjQ3MDA6MzA6OjY4MWM6MWQyMl0gT7VsEyKePk-NGW4wncgn3krUtflpClvMP1lhELmyackOZG9oLnRpYXJhcC5vcmcKL2Rucy1xdWVyeQ",
        "sdns://AQcAAAAAAAAAFDIwNi4xODkuMTQyLjE3OTo1MzUzII5GJ8c4g6hRAwghulrn5dBB9KrvlbeCkBbLZR2HwyjJGTIuZG5zY3J5cHQtY2VydC5hcnZpbmQuaW8",
        "sdns://AQcAAAAAAAAAEzE3OC4xMjguMjU1LjI4OjUzNTMgkr1k-Lp2d9IXiFlXoBAgFGZUCJSPW_x81Ec6ShkPsJYdMi5kbnNjcnlwdC1jZXJ0LmJvdHRsZXBvc3QubWU",
        "sdns://AgMAAAAAAAAADjEzNC4yMDkuMTQ2LjE2oD4aGg9sU_PpekktVwhLW5gHBZ7gV6sVBYdv2D_aPbg4ID4aGg9sU_PpekktVwhLW5gHBZ7gV6sVBYdv2D_aPbg4EGRucy5icmFobWEud29ybGQKL2Rucy1xdWVyeQ",
        "sdns://AQQAAAAAAAAAEjEzOS41OS40OC4yMjI6NDQzNCAFOt_yxaMpFtga2IpneSwwK6rV0oAyleham9IvhoceEBsyLmRuc2NyeXB0LWNlcnQuY2FwdG5lbW8uaW4",
        "sdns://AQcAAAAAAAAAETUxLjE1LjEwNi4xNzY6NDQzIEvW6V2Vu6gTWx8Go_PTrPTAjz1-d_Gd9A8KsOPx5NcRGjIuZG5zY3J5cHQtY2VydC5pdC5lZG8uY29t",
        "sdns://AQAAAAAAAAAADjIwOC42Ny4yMjAuMjIwILc1EUAgbyJdPivYItf9aR6hwzzI1maNDL4Ev6vKQ_t5GzIuZG5zY3J5cHQtY2VydC5vcGVuZG5zLmNvbQ",
        "sdns://AQAAAAAAAAAADjIwOC42Ny4yMjAuMTIzILc1EUAgbyJdPivYItf9aR6hwzzI1maNDL4Ev6vKQ_t5GzIuZG5zY3J5cHQtY2VydC5vcGVuZG5zLmNvbQ",
        "sdns://AQAAAAAAAAAAD1syNjIwOjA6Y2NjOjoyXSC3NRFAIG8iXT4r2CLX_WkeocM8yNZmjQy-BL-rykP7eRsyLmRuc2NyeXB0LWNlcnQub3BlbmRucy5jb20",
        "sdns://AQMAAAAAAAAAEzE4NS4yMjguMTY4LjEwOjg0NDMgvKwy-tVDaRcfCDLWB1AnwyCM7vDo6Z-UGNx3YGXUjykRY2xlYW5icm93c2luZy5vcmc",
        "sdns://AQMAAAAAAAAAFVsyYTBkOjJhMDA6MTo6MV06ODQ0MyC8rDL61UNpFx8IMtYHUCfDIIzu8Ojpn5QY3HdgZdSPKRFjbGVhbmJyb3dzaW5nLm9yZw",
        "sdns://AQMAAAAAAAAAFDE4NS4yMjguMTY4LjE2ODo4NDQzILysMvrVQ2kXHwgy1gdQJ8MgjO7w6OmflBjcd2Bl1I8pEWNsZWFuYnJvd3Npbmcub3Jn",
        "sdns://AQMAAAAAAAAAFFsyYTBkOjJhMDA6MTo6XTo4NDQzILysMvrVQ2kXHwgy1gdQJ8MgjO7w6OmflBjcd2Bl1I8pEWNsZWFuYnJvd3Npbmcub3Jn",
        "sdns://AQcAAAAAAAAADjE2NC4xMzIuNDUuMTEyIJjcR4lfaiSu1VnGEUxuFHjZAJMtzh4IIzY1nENSdINmIDIuZG5zY3J5cHQtY2VydC5uczEuZGV2ZWxvcGVyLmxp",
        "sdns://AQcAAAAAAAAAHVsyMDAxOjQxZDA6MzAyOjIyMDA6OjczNV06NDQzIOORyBsbVECZjBLtBBPcvz4BX6KqFLyBHOgRFnPjDc91IDIuZG5zY3J5cHQtY2VydC5uczEuZGV2ZWxvcGVyLmxp",
        "sdns://AQcAAAAAAAAACzUxLjg5LjIyLjM2IFCr7Cwmkxqhwih6Yd6DXna4QnD43eEobcm4jYfdOKGQIDIuZG5zY3J5cHQtY2VydC5uczIuZGV2ZWxvcGVyLmxp",
        "sdns://AQcAAAAAAAAAHlsyMDAxOjQxZDA6NzAxOjExMDA6OjIxYjVdOjQ0MyBF27xjwGrNk7IfBlWNxDMfThgQaJl-G-B6V-ORq6ZblSAyLmRuc2NyeXB0LWNlcnQubnMyLmRldmVsb3Blci5saQ",
        "sdns://AgcAAAAAAAAADjE2NC4xMzIuNDUuMTEyABVkbnMuZGV2ZWxvcGVyLmxpOjg0NDMKL2Rucy1xdWVyeQ",
        "sdns://AgcAAAAAAAAAHlsyMDAxOjQxZDA6MzAyOjIyMDA6OjczNV06ODQ0MwAVZG5zLmRldmVsb3Blci5saTo4NDQzCi9kbnMtcXVlcnk",
        "sdns://AgcAAAAAAAAACzUxLjg5LjIyLjM2ABZkbnMyLmRldmVsb3Blci5saTo4NDQzCi9kbnMtcXVlcnk",
        "sdns://AgcAAAAAAAAAH1syMDAxOjQxZDA6NzAxOjExMDA6OjIxYjVdOjg0NDMAFmRuczIuZGV2ZWxvcGVyLmxpOjg0NDMKL2Rucy1xdWVyeQ",
        "sdns://AgcAAAAAAAAAAKA-GhoPbFPz6XpJLVcIS1uYBwWe4FerFQWHb9g_2j24OKBoo-sB-l8CxNNfOhHQBMrwiyJL7qfXnFiMfxPIYTNgLqDvR4Wu5wydV1_nM4MG2T6nlhHl_tzvU2LdZsmLYLstvSAcVDa2UaK1QVwWz9ltGpcJ_ZyPJ-73XPlz2YL_5o5Y8BxkbnMuZGlnaXRhbGUtZ2VzZWxsc2NoYWZ0LmNoCi9kbnMtcXVlcnk",
        "sdns://AQcAAAAAAAAAEjE5Mi45OS4xODMuMTMyOjQ0MyAaU6PJUHicvdELGTOkaJtshGpA8bc9F1KuysmCnst84h0yLmRuc2NyeXB0LWNlcnQuZG5zY3J5cHQuY2EtMQ",
        "sdns://AQcAAAAAAAAAETE0OS41Ni4yMjguNDU6NDQzIAEIVKs7Vqfu-dORWP72ggv_k6I1fIkWCNueFdO74BGFHTIuZG5zY3J5cHQtY2VydC5kbnNjcnlwdC5jYS0y",
        "sdns://AQcAAAAAAAAAHFsyNjA3OjUzMDA6NjA6NGFhODo6NjAwXTo0NDMgINkZ1Ow8UAjNxwpR6itPy_6KmTxkMdDsaB1epzhFq4AiMi5kbnNjcnlwdC1jZXJ0LmRuc2NyeXB0LmNhLTEtaXB2Ng",
        "sdns://AQcAAAAAAAAAHFsyNjA3OjUzMDA6MTIwOmI5Yjo6MjAwXTo0NDMgmHxwqJfb2hUaNK1LVeqADvVhzASq1cV90sPYYfwX9CkiMi5kbnNjcnlwdC1jZXJ0LmRuc2NyeXB0LmNhLTItaXB2Ng",
        "sdns://AQcAAAAAAAAAEzExNi4yMDMuNzAuMTU2Ojg0NDMgenKjVeH-LU7Opsyq1ljKZz14fHsngOK8OOeQ-cR2mAskMi5kbnNjcnlwdC1jZXJ0LmRuc3dhcmRlbi0xLXVuY2Vuc29y",
        "sdns://AQcAAAAAAAAAHFsyYTAxOjRmODoxYzFjOjc1YjQ6OjFdOjg0NDMgenKjVeH-LU7Opsyq1ljKZz14fHsngOK8OOeQ-cR2mAskMi5kbnNjcnlwdC1jZXJ0LmRuc3dhcmRlbi0xLXVuY2Vuc29y",
        "sdns://AQcAAAAAAAAAEzExNi4yMDMuMzUuMjU1Ojg0NDMg-IlTTFFgMuUntnNV78COzbhJN9_OEOVWNgHhdg4BNXwkMi5kbnNjcnlwdC1jZXJ0LmRuc3dhcmRlbi0yLXVuY2Vuc29y",
        "sdns://AQcAAAAAAAAAHFsyYTAxOjRmODoxYzFjOjVlNzc6OjFdOjg0NDMg-IlTTFFgMuUntnNV78COzbhJN9_OEOVWNgHhdg4BNXwkMi5kbnNjcnlwdC1jZXJ0LmRuc3dhcmRlbi0yLXVuY2Vuc29y",
        "sdns://AgcAAAAAAAAADjExNi4yMDMuNzAuMTU2ID4aGg9sU_PpekktVwhLW5gHBZ7gV6sVBYdv2D_aPbg4EWRvaC5kbnN3YXJkZW4uY29tCy91bmNlbnNvcmVk",
        "sdns://AgcAAAAAAAAAF1syYTAxOjRmODoxYzFjOjc1YjQ6OjFdID4aGg9sU_PpekktVwhLW5gHBZ7gV6sVBYdv2D_aPbg4EWRvaC5kbnN3YXJkZW4uY29tCy91bmNlbnNvcmVk",
        "sdns://AgcAAAAAAAAADjExNi4yMDMuMzUuMjU1ID4aGg9sU_PpekktVwhLW5gHBZ7gV6sVBYdv2D_aPbg4EWRvaC5kbnN3YXJkZW4uY29tCy91bmNlbnNvcmVk",
        "sdns://AgcAAAAAAAAAF1syYTAxOjRmODoxYzFjOjVlNzc6OjFdID4aGg9sU_PpekktVwhLW5gHBZ7gV6sVBYdv2D_aPbg4EWRvaC5kbnN3YXJkZW4uY29tCy91bmNlbnNvcmVk",
        "sdns://AQMAAAAAAAAAEjE4NS4yMjguMTY4Ljk6ODQ0MyC8rDL61UNpFx8IMtYHUCfDIIzu8Ojpn5QY3HdgZdSPKRFjbGVhbmJyb3dzaW5nLm9yZw",
        "sdns://AgcAAAAAAAAABzEuMC4wLjGgENk8mGSlIfMGXMOlIlCcKvq7AVgcrZxtjon911-ep0cg63Ul-I8NlFj4GplQGb_TTLiczclX57DvMV8Q-JdjgRgSZG5zLmNsb3VkZmxhcmUuY29tCi9kbnMtcXVlcnk",
        "sdns://AgcAAAAAAAAAGVsyNjA2OjQ3MDA6NDcwMDo6MTExMV06NTOgENk8mGSlIfMGXMOlIlCcKvq7AVgcrZxtjon911-ep0cg63Ul-I8NlFj4GplQGb_TTLiczclX57DvMV8Q-JdjgRgSZG5zLmNsb3VkZmxhcmUuY29tCi9kbnMtcXVlcnk",
        "sdns://AgMAAAAAAAAADDQ1Ljc3LjE4MC4xMCBsA2QQ3lR1Nl9Ygfr8FdBIpL-doxmHECRx3T5NIXYYtxNkbnMuY29udGFpbmVycGkuY29tCi9kbnMtcXVlcnk",
        "sdns://AgMAAAAAAAAALVsyMDAxOjE5ZjA6NzAwMTo1NTU0OjU0MDA6MDJmZjpmZTU3OjMwNzddOjQ0MyBsA2QQ3lR1Nl9Ygfr8FdBIpL-doxmHECRx3T5NIXYYtxNkbnMuY29udGFpbmVycGkuY29tCi9kbnMtcXVlcnk",
        "sdns://AgUAAAAAAAAAACA-GhoPbFPz6XpJLVcIS1uYBwWe4FerFQWHb9g_2j24OAxjb21tb25zLmhvc3QKL2Rucy1xdWVyeQ",
        "sdns://AQUAAAAAAAAACjguMjAuMjQ3LjIg0sJUqpYcHsoXmZb1X7yAHwg2xyN5q1J-zaiGG-Dgs7AoMi5kbnNjcnlwdC1jZXJ0LnNoaWVsZC0yLmRuc2J5Y29tb2RvLmNvbQ",
        "sdns://AQYAAAAAAAAACzgxLjE3LjMxLjM0IDEzcq1ZVjLCQWuHLwmPhRvduWUoTGy-mk8ZCWQw26laHjIuZG5zY3J5cHQtY2VydC5jcnlwdG9zdG9ybS5pcw",
        "sdns://AQYAAAAAAAAADzEyOC4xMjcuMTA0LjEwOCAxM3KtWVYywkFrhy8Jj4Ub3bllKExsvppPGQlkMNupWh4yLmRuc2NyeXB0LWNlcnQuY3J5cHRvc3Rvcm0uaXM",
        "sdns://AQYAAAAAAAAADjIxMy4xNjMuNjQuMjA4IDEzcq1ZVjLCQWuHLwmPhRvduWUoTGy-mk8ZCWQw26laHjIuZG5zY3J5cHQtY2VydC5jcnlwdG9zdG9ybS5pcw",
        "sdns://AQYAAAAAAAAADTE4NS4xMDcuODAuODQgMTNyrVlWMsJBa4cvCY-FG925ZShMbL6aTxkJZDDbqVoeMi5kbnNjcnlwdC1jZXJ0LmNyeXB0b3N0b3JtLmlz",
        "sdns://AQYAAAAAAAAADjE4NS4xMTcuMTE4LjIwIDEzcq1ZVjLCQWuHLwmPhRvduWUoTGy-mk8ZCWQw26laHjIuZG5zY3J5cHQtY2VydC5jcnlwdG9zdG9ybS5pcw",
        "sdns://AQYAAAAAAAAACzUuMTMzLjguMTg3IDEzcq1ZVjLCQWuHLwmPhRvduWUoTGy-mk8ZCWQw26laHjIuZG5zY3J5cHQtY2VydC5jcnlwdG9zdG9ybS5pcw",
        "sdns://AQYAAAAAAAAADjE4NS45NC4xOTMuMjM0IDEzcq1ZVjLCQWuHLwmPhRvduWUoTGy-mk8ZCWQw26laHjIuZG5zY3J5cHQtY2VydC5jcnlwdG9zdG9ybS5pcw",
        "sdns://AQYAAAAAAAAADTIxMi4xMjkuNDYuMzIgMTNyrVlWMsJBa4cvCY-FG925ZShMbL6aTxkJZDDbqVoeMi5kbnNjcnlwdC1jZXJ0LmNyeXB0b3N0b3JtLmlz",
        "sdns://AQYAAAAAAAAADTE5NS4xNTQuNDAuNDggMTNyrVlWMsJBa4cvCY-FG925ZShMbL6aTxkJZDDbqVoeMi5kbnNjcnlwdC1jZXJ0LmNyeXB0b3N0b3JtLmlz",
        "sdns://AQYAAAAAAAAADTEwOS43MS40Mi4yMjggMTNyrVlWMsJBa4cvCY-FG925ZShMbL6aTxkJZDDbqVoeMi5kbnNjcnlwdC1jZXJ0LmNyeXB0b3N0b3JtLmlz",
        "sdns://AQYAAAAAAAAADDUuMjU0Ljk2LjE5NSAxM3KtWVYywkFrhy8Jj4Ub3bllKExsvppPGQlkMNupWh4yLmRuc2NyeXB0LWNlcnQuY3J5cHRvc3Rvcm0uaXM",
        "sdns://AQYAAAAAAAAADzE3OC4xNzUuMTM5LjIxMSAxM3KtWVYywkFrhy8Jj4Ub3bllKExsvppPGQlkMNupWh4yLmRuc2NyeXB0LWNlcnQuY3J5cHRvc3Rvcm0uaXM",
        "sdns://AQYAAAAAAAAADzEwOS4yNDguMTQ5LjEzMyAxM3KtWVYywkFrhy8Jj4Ub3bllKExsvppPGQlkMNupWh4yLmRuc2NyeXB0LWNlcnQuY3J5cHRvc3Rvcm0uaXM",
        "sdns://AQYAAAAAAAAADTgyLjE2My43Mi4xMjMgMTNyrVlWMsJBa4cvCY-FG925ZShMbL6aTxkJZDDbqVoeMi5kbnNjcnlwdC1jZXJ0LmNyeXB0b3N0b3JtLmlz",
        "sdns://AQYAAAAAAAAADDg0LjE2LjI0MC40MyAxM3KtWVYywkFrhy8Jj4Ub3bllKExsvppPGQlkMNupWh4yLmRuc2NyeXB0LWNlcnQuY3J5cHRvc3Rvcm0uaXM",
        "sdns://AQYAAAAAAAAADjg5LjE2My4yMTQuMTc0IDEzcq1ZVjLCQWuHLwmPhRvduWUoTGy-mk8ZCWQw26laHjIuZG5zY3J5cHQtY2VydC5jcnlwdG9zdG9ybS5pcw",
        "sdns://AQYAAAAAAAAADzE2Mi4yMjEuMjA3LjIyOCAxM3KtWVYywkFrhy8Jj4Ub3bllKExsvppPGQlkMNupWh4yLmRuc2NyeXB0LWNlcnQuY3J5cHRvc3Rvcm0uaXM",
        "sdns://AQYAAAAAAAAADjE2Ny4xMTQuODQuMTMyIDEzcq1ZVjLCQWuHLwmPhRvduWUoTGy-mk8ZCWQw26laHjIuZG5zY3J5cHQtY2VydC5jcnlwdG9zdG9ybS5pcw",
        "sdns://AQYAAAAAAAAADzE3My4yMzQuMTU5LjIzNSAxM3KtWVYywkFrhy8Jj4Ub3bllKExsvppPGQlkMNupWh4yLmRuc2NyeXB0LWNlcnQuY3J5cHRvc3Rvcm0uaXM",
        "sdns://AQYAAAAAAAAADjE3My4yMzQuNTYuMTE1IDEzcq1ZVjLCQWuHLwmPhRvduWUoTGy-mk8ZCWQw26laHjIuZG5zY3J5cHQtY2VydC5jcnlwdG9zdG9ybS5pcw",
        "sdns://AQYAAAAAAAAADDY0LjEyMC41LjI1MSAxM3KtWVYywkFrhy8Jj4Ub3bllKExsvppPGQlkMNupWh4yLmRuc2NyeXB0LWNlcnQuY3J5cHRvc3Rvcm0uaXM",
        "sdns://AQYAAAAAAAAADDE5OC43LjU4LjIyNyAxM3KtWVYywkFrhy8Jj4Ub3bllKExsvppPGQlkMNupWh4yLmRuc2NyeXB0LWNlcnQuY3J5cHRvc3Rvcm0uaXM",
        "sdns://AQYAAAAAAAAADTIwOS41OC4xNDcuMzYgMTNyrVlWMsJBa4cvCY-FG925ZShMbL6aTxkJZDDbqVoeMi5kbnNjcnlwdC1jZXJ0LmNyeXB0b3N0b3JtLmlz",
        "sdns://AQYAAAAAAAAADTY0LjQyLjE4MS4yMjcgMTNyrVlWMsJBa4cvCY-FG925ZShMbL6aTxkJZDDbqVoeMi5kbnNjcnlwdC1jZXJ0LmNyeXB0b3N0b3JtLmlz",
        "sdns://AQYAAAAAAAAADjE1NS4yNTQuMjkuMTEzIDEzcq1ZVjLCQWuHLwmPhRvduWUoTGy-mk8ZCWQw26laHjIuZG5zY3J5cHQtY2VydC5jcnlwdG9zdG9ybS5pcw",
        "sdns://AQYAAAAAAAAADDIzLjE5LjY3LjExNiAxM3KtWVYywkFrhy8Jj4Ub3bllKExsvppPGQlkMNupWh4yLmRuc2NyeXB0LWNlcnQuY3J5cHRvc3Rvcm0uaXM",
        "sdns://AQYAAAAAAAAADTEwNC4yNTUuMTc1LjIgMTNyrVlWMsJBa4cvCY-FG925ZShMbL6aTxkJZDDbqVoeMi5kbnNjcnlwdC1jZXJ0LmNyeXB0b3N0b3JtLmlz",
        "sdns://AQcAAAAAAAAADTkzLjk1LjIyNi4xNjUghGA0qcYwyjwErEqQFiXxeoeyrLlBgKxIHiwQ6M7eGm8cMi5kbnNjcnlwdC1jZXJ0LmlzMi5kMHduLmJpeg",
        "sdns://AQcAAAAAAAAACzQxLjc5LjY5LjEzINYGFfvRRTuhTnaKPlxcs6wXRhMxRj2gr4z33wTaTXVtGzIuZG5zY3J5cHQtY2VydC50ei5kMHduLmJpeg",
        "sdns://AQcAAAAAAAAAGFsyYzBmOmZkYTg6NTo6MmVkMTpkMmVjXSDWBhX70UU7oU52ij5cXLOsF0YTMUY9oK-M998E2k11bRsyLmRuc2NyeXB0LWNlcnQudHouZDB3bi5iaXo",
        "sdns://AQcAAAAAAAAAEzIwOS4yNTAuMjM1LjE3MDo0NDMgz0wbvISl_NVCSe0wDJMS79BAFZoWth1djmhuzv_n3KAiMi5kbnNjcnlwdC1jZXJ0LmRlLmRuc21hc2NoaW5lLm5ldA",
        "sdns://AQcAAAAAAAAADDc3LjY2Ljg0LjIzMyA3SFWF47nQiP0lrTawNwH1UgzWSJ6a3VIUV0lVnwqZVSUyLmRuc2NyeXB0LWNlcnQucmVzb2x2ZXIyLmRuc2NyeXB0LmV1",
        "sdns://AQcAAAAAAAAAFFsyMDAxOjE0NDg6MjQzOjpkYzJdIDdIVYXjudCI_SWtNrA3AfVSDNZInprdUhRXSVWfCplVJTIuZG5zY3J5cHQtY2VydC5yZXNvbHZlcjIuZG5zY3J5cHQuZXU",
        "sdns://AQcAAAAAAAAADjE3Ni41Ni4yMzcuMTcxIGfADywhxVSBRd18tGonGvLrlpkxQKMJtiuNFlMRhZxmJTIuZG5zY3J5cHQtY2VydC5yZXNvbHZlcjEuZG5zY3J5cHQuZXU",
        "sdns://AQcAAAAAAAAADDQ1Ljc2LjM1LjIxMiBMhPuMBRFd-l-Xxe0DKRNwx4q81k4V3VOrCN5y-4RKyh8yLmRuc2NyeXB0LWNlcnQubnMwLmRuc2NyeXB0Lm5s",
        "sdns://AQcAAAAAAAAAEjEzOS41OS4yMDAuMTE2OjQ0MyAmJwT-OXZ9NntZ2eu_HtZeXARhCdiAynbBYcu6bArCdxsyLmRuc2NyeXB0LWNlcnQuZG5zY3J5cHQudWs",
        "sdns://AQcAAAAAAAAAHlsyYTAzOmIwYzA6MTplMDo6MmUzOmUwMDFdOjQ0MyAmJwT-OXZ9NntZ2eu_HtZeXARhCdiAynbBYcu6bArCdxsyLmRuc2NyeXB0LWNlcnQuZG5zY3J5cHQudWs",
        "sdns://AQcAAAAAAAAAETk0LjEzMC4xNzguNTY6NDQzIIxpj-7XPoT_79rA9pnvVGz0bIQRuEL-eI-0NlYJaGcpJjIuZG5zY3J5cHQtY2VydC5kbnNjcnlwdC0wMS5tYWRwb255Lmlv",
        "sdns://AQcAAAAAAAAAETE0MC44Mi4yNi4xMDM6NDQzIE15px_otxEaCZ20DybtbfMu92IH3Ritg83ibv6LeizTKTIuZG5zY3J5cHQtY2VydC5kbnNjcnlwdC0wMi5tYWRwb255LnNwYWNl",
        "sdns://AQcAAAAAAAAAEDUuMTg4LjIzOC42ODo0NDMg1uv1UTjfRdCF1XDI3T10v4EXWcdK6x8qM5Qut7bwb_gpMi5kbnNjcnlwdC1jZXJ0LmRuc2NyeXB0LTAzLm1hZHBvbnkuc3BhY2U",
        "sdns://AQcAAAAAAAAAEDQ1LjMyLjMxLjIzMTo0NDMgmk18Se_bsOdRNFJ64Lrw5MJ83y_au6WNrh3lZOceiqgpMi5kbnNjcnlwdC1jZXJ0LmRuc2NyeXB0LTA0Lm1hZHBvbnkuc3BhY2U",
        "sdns://AQcAAAAAAAAAETE0OS4yOC4xNjguNjI6NDQzIENfI6UCxKdNccBA9YW-OhkV-HB_b_Yj5nQbq-gM1TAMKTIuZG5zY3J5cHQtY2VydC5kbnNjcnlwdC0wNS5tYWRwb255LnNwYWNl",
        "sdns://AQcAAAAAAAAAEjk1LjE3OS4xNzguMTAwOjQ0MyCzDTlSDfD9-UOciubW46-f6tsh8o60Rt1m4i7XH5hBqykyLmRuc2NyeXB0LWNlcnQuZG5zY3J5cHQtMDYubWFkcG9ueS5zcGFjZQ",
        "sdns://AQcAAAAAAAAAEjEzOS4xODAuMjE2LjgzOjQ0MyBPxDlEgU5vJPp0n-Zh505hgFMSBQj8CQc7p9uUaIWigSkyLmRuc2NyeXB0LWNlcnQuZG5zY3J5cHQtMDcubWFkcG9ueS5zcGFjZQ",
        "sdns://AQMAAAAAAAAAEzk1LjIxNi4yMTIuMTc3Ojg0NDMgU4ToFEMUKT5W3RsUCh7xcq1HvboXmciVcpSVPQNOtccbMi5kbnNjcnlwdC1jZXJ0LmJsYWhkbnMuY29t",
        "sdns://AQMAAAAAAAAAHFsyYTAxOjRmOTpjMDEwOjQzY2U6OjFdOjg0NDMgU4ToFEMUKT5W3RsUCh7xcq1HvboXmciVcpSVPQNOtccbMi5kbnNjcnlwdC1jZXJ0LmJsYWhkbnMuY29t",
        "sdns://AQMAAAAAAAAAJVsyYTBhOmU1YzA6MjoyOjA6YzhmZjpmZTY4OmJmNDhdOjg0NDMgU4ToFEMUKT5W3RsUCh7xcq1HvboXmciVcpSVPQNOtccbMi5kbnNjcnlwdC1jZXJ0LmJsYWhkbnMuY29t",
        "sdns://AQMAAAAAAAAAEzEwOC42MS4yMDEuMTE5Ojg0NDMgU4ToFEMUKT5W3RsUCh7xcq1HvboXmciVcpSVPQNOtccbMi5kbnNjcnlwdC1jZXJ0LmJsYWhkbnMuY29t",
        "sdns://AQMAAAAAAAAALlsyMDAxOjE5ZjA6NzAwMToxZGVkOjU0MDA6MDFmZjpmZTkwOjk0NWJdOjg0NDMgU4ToFEMUKT5W3RsUCh7xcq1HvboXmciVcpSVPQNOtccbMi5kbnNjcnlwdC1jZXJ0LmJsYWhkbnMuY29t",
        "sdns://AQMAAAAAAAAAEzE1OS42OS4xOTguMTAxOjg0NDMgU4ToFEMUKT5W3RsUCh7xcq1HvboXmciVcpSVPQNOtccbMi5kbnNjcnlwdC1jZXJ0LmJsYWhkbnMuY29t",
        "sdns://AQMAAAAAAAAAHFsyYTAxOjRmODoxYzFjOjZiNGI6OjFdOjg0NDMgU4ToFEMUKT5W3RsUCh7xcq1HvboXmciVcpSVPQNOtccbMi5kbnNjcnlwdC1jZXJ0LmJsYWhkbnMuY29t",
        "sdns://AgUAAAAAAAAAACA9pLcWNKQTwc7zSqltJaQBY01M82w7Ezx0KU5I5jcBKgpkb2guZG5zLnNiCi9kbnMtcXVlcnk",
        "sdns://AgMAAAAAAAAAF1syYTAxOjRmOTpjMDEwOjQzY2U6OjFdABJkb2gtZmkuYmxhaGRucy5jb20KL2Rucy1xdWVyeQ",
        "sdns://AQMAAAAAAAAAEzk1LjIxNi4yMTIuMTc3Ojg0NDMgU4ToFEMUKT5W3RsUCh7xcq1HvboXmciVcpSVPQNOtccbMi5kbnNjcnlwdC1jZXJ0LmJsYWhkbnMuY29t",
        "sdns://AgMAAAAAAAAAIFsyYTBhOmU1YzA6MjoyOjA6YzhmZjpmZTY4OmJmNDhdABJkb2gtY2guYmxhaGRucy5jb20KL2Rucy1xdWVyeQ",
        "sdns://AgMAAAAAAAAADjE1OS42OS4xOTguMTAxABJkb2gtZGUuYmxhaGRucy5jb20KL2Rucy1xdWVyeQ",
        "sdns://AgMAAAAAAAAAF1syYTAxOjRmODoxYzFjOjZiNGI6OjFdABJkb2gtZGUuYmxhaGRucy5jb20KL2Rucy1xdWVyeQ",
        "sdns://AgMAAAAAAAAADjEwOC42MS4yMDEuMTE5ABJkb2gtanAuYmxhaGRucy5jb20KL2Rucy1xdWVyeQ",
        "sdns://AgMAAAAAAAAAKVsyMDAxOjE5ZjA6NzAwMToxZGVkOjU0MDA6MDFmZjpmZTkwOjk0NWJdABJkb2gtanAuYmxhaGRucy5jb20KL2Rucy1xdWVyeQ",
        "sdns://AgMAAAAAAAAAAAAVZG9oLmNsZWFuYnJvd3Npbmcub3JnEi9kb2gvYWR1bHQtZmlsdGVyLw",
        "sdns://AgMAAAAAAAAAAAAVZG9oLmNsZWFuYnJvd3Npbmcub3JnEy9kb2gvZmFtaWx5LWZpbHRlci8",
        "sdns://AgMAAAAAAAAAAAAVZG9oLmNsZWFuYnJvd3Npbmcub3JnFS9kb2gvc2VjdXJpdHktZmlsdGVyLw",
        "sdns://AgcAAAAAAAAADDEwNC4yOC4wLjEwNiAd2FCKjFZZBDl8eGRR4I9XYTzzyKcj9vN5_Uw4WLbznw1kb2guY3J5cHRvLnN4Ci9kbnMtcXVlcnk",
        "sdns://AgcAAAAAAAAAF1syNjA2OjQ3MDA6MzA6OjY4MWM6NmFdIB3YUIqMVlkEOXx4ZFHgj1dhPPPIpyP283n9TDhYtvOfEmRvaC1pcHY2LmNyeXB0by5zeAovZG5zLXF1ZXJ5",
        "sdns://AgcAAAAAAAAAACA-GhoPbFPz6XpJLVcIS1uYBwWe4FerFQWHb9g_2j24OBRpYmtzdHVybS5zeW5vbG9neS5tZQovZG5zLXF1ZXJ5",
        "sdns://AQcAAAAAAAAADjEwNC4zNi4xNDkuMTc3INRzo8srd84NzJJs_OGLkPMBC2jBNVH_U4US4ca7DXRXHTIuZG5zY3J5cHQtY2VydC5ldmlsdmliZXMuY29t",
        "sdns://AgcAAAAAAAAADDE5NS4zMC45NC4yOCA-GhoPbFPz6XpJLVcIS1uYBwWe4FerFQWHb9g_2j24OA1kb2guZmZtdWMubmV0Ci9kbnMtcXVlcnk",
        "sdns://AgcAAAAAAAAADDE5NS4zMC45NC4yOAANZG9oLmZmbXVjLm5ldAovZG5zLXF1ZXJ5",
        "sdns://AgcAAAAAAAAAFVsyMDAxOjYwODphMDE6OjNdOjQ0MyA-GhoPbFPz6XpJLVcIS1uYBwWe4FerFQWHb9g_2j24OA1kb2guZmZtdWMubmV0Ci9kbnMtcXVlcnk",
        "sdns://AQcAAAAAAAAAEzIwNS4xODUuMTE2LjExNjo1NTMg2P-7QuAxvnp5cwtFVo1Jak6Ky1mqg2b9arkeJyp9FuQbMi5kbnNjcnlwdC1jZXJ0LmZyZWV0c2Eub3Jn",
        "sdns://AQMAAAAAAAAAETExOC4yNC4yMDguMTk3OjIyIL66dzE0aNGlvYsF3RnukLk4AI3lQqSxeEo6PxHme-qZGTIuZG5zY3J5cHQtY2VydC4yMzNweS5jb20",
        "sdns://AQMAAAAAAAAAEDExOS4yOS4xMDcuODU6MjIgCsUpkBu6ujbEDZ8PQI2wa3DCkDzxLlczLOwTyQTQM70ZMi5kbnNjcnlwdC1jZXJ0LjIzM3B5LmNvbQ",
        "sdns://AQMAAAAAAAAAEjExNC4xMTUuMjQwLjE3NToyMiCLntDYEK0AwismFtMCM0YMkflJGNZiJnINFtDLcCLLwBkyLmRuc2NyeXB0LWNlcnQuMjMzcHkuY29t",
        "sdns://AQcAAAAAAAAAETE1MC4xMDkuNzQuMTY0OjIyIPcEBXJOU2jQys6br08P8yyn132SuDixQ8Oek3lhoRQoGTIuZG5zY3J5cHQtY2VydC4yMzNweS5jb20",
        "sdns://AgcAAAAAAAAADTQ3LjEwMS4xMzYuMzcgot4zSQxHbTVuLbxzfCd5aSJJUmtlq4-6mjQoBIHIvfwOZWRucy4yMzNweS5jb20KL2Rucy1xdWVyeQ",
        "sdns://AgMAAAAAAAAADjExOC4yNC4yMDguMTk3ID4aGg9sU_PpekktVwhLW5gHBZ7gV6sVBYdv2D_aPbg4DndkbnMuMjMzcHkuY29tCi9kbnMtcXVlcnk",
        "sdns://AgcAAAAAAAAADTExOS4yOS4xMDcuODUgot4zSQxHbTVuLbxzfCd5aSJJUmtlq4-6mjQoBIHIvfwOc2Rucy4yMzNweS5jb20KL2Rucy1xdWVyeQ",
        "sdns://AgcAAAAAAAAADzExNC4xMTUuMjQwLjE3NSCi3jNJDEdtNW4tvHN8J3lpIklSa2Wrj7qaNCgEgci9_A5uZG5zLjIzM3B5LmNvbQovZG5zLXF1ZXJ5",
        "sdns://AgUAAAAAAAAABzguOC44LjigHvYkz_9ea9O63fP92_3qVlRn43cpncfuZnUWbzAMwbkgdoAkR6AZkxo_AEMExT_cbBssN43Evo9zs5_ZyWnftEUKZG5zLmdvb2dsZQovZG5zLXF1ZXJ5",
        "sdns://AgcAAAAAAAAADjE3Mi4xMDUuMjQxLjkzAA1qcC5ncmlkbnMueHl6Ci9kbnMtcXVlcnk",
        "sdns://AgcAAAAAAAAAIFsyNDAwOjg5MDI6OmYwM2M6OTFmZjpmZWVkOjIyMGJdAA1qcC5ncmlkbnMueHl6Ci9kbnMtcXVlcnk",
        "sdns://AgcAAAAAAAAADTEzOS4xNjIuMy4xMjMADXNnLmdyaWRucy54eXoKL2Rucy1xdWVyeQ",
        "sdns://AgcAAAAAAAAAIFsyNDAwOjg5MDE6OmYwM2M6OTFmZjpmZWVkOjhkNDddAA1zZy5ncmlkbnMueHl6Ci9kbnMtcXVlcnk",
        "sdns://AQcAAAAAAAAADzgzLjc3Ljg1Ljc6ODQ0MyC33sjOthdoqrx-so3KV8EGejv9ZmHp1Y8ogm7dvAAKpRgyLmRuc2NyeXB0LWNlcnQuaWJrc3R1cm0",
        "sdns://AQMAAAAAAAAADjE3NC4xMzguMjEuMTI4IO-WgGbo2ZTwZdg-3dMa7u31bYZXRj5KykfN1_6Xw9T2HDIuZG5zY3J5cHQtY2VydC5kbnMudGlhci5hcHA",
        "sdns://AQMAAAAAAAAAG1syNDAwOjYxODA6MDpkMDo6NWY2ZTo0MDAxXSDvloBm6NmU8GXYPt3TGu7t9W2GV0Y-SspHzdf-l8PU9hwyLmRuc2NyeXB0LWNlcnQuZG5zLnRpYXIuYXBw",
        "sdns://AgMAAAAAAAAADjE3NC4xMzguMjkuMTc1ID4aGg9sU_PpekktVwhLW5gHBZ7gV6sVBYdv2D_aPbg4DGRvaC50aWFyLmFwcAovZG5zLXF1ZXJ5",
        "sdns://AgMAAAAAAAAAG1syNDAwOjYxODA6MDpkMDo6NWY3Mzo0MDAxXSA-GhoPbFPz6XpJLVcIS1uYBwWe4FerFQWHb9g_2j24OAxkb2gudGlhci5hcHAKL2Rucy1xdWVyeQ",
        "sdns://AgUAAAAAAAAACjEwMy4yLjU3LjUgs2lfGAQCPrV0DPQqOkPci0Jei0GhMK_ne-QHwPbfn4oRcHVibGljLmRucy5paWouanAKL2Rucy1xdWVyeQ",
        "sdns://AQcAAAAAAAAAEjE3Mi4xMDQuOTMuODA6MTQ0MyAyuHY-8b9lNqHeahPAzW9IoXnjiLaZpTeNbVs8TN9UUxsyLmRuc2NyeXB0LWNlcnQuanAudGlhci5hcHA",
        "sdns://AQcAAAAAAAAAJVsyNDAwOjg5MDI6OmYwM2M6OTFmZjpmZWRhOmM1MTRdOjE0NDMgMrh2PvG_ZTah3moTwM1vSKF544i2maU3jW1bPEzfVFMbMi5kbnNjcnlwdC1jZXJ0LmpwLnRpYXIuYXBw",
        "sdns://AgcAAAAAAAAADTE3Mi4xMDQuOTMuODAgPhoaD2xT8-l6SS1XCEtbmAcFnuBXqxUFh2_YP9o9uDgLanAudGlhci5hcHAKL2Rucy1xdWVyeQ",
        "sdns://AgcAAAAAAAAAIFsyNDAwOjg5MDI6OmYwM2M6OTFmZjpmZWRhOmM1MTRdID4aGg9sU_PpekktVwhLW5gHBZ7gV6sVBYdv2D_aPbg4C2pwLnRpYXIuYXBwCi9kbnMtcXVlcnk",
        "sdns://AgcAAAAAAAAADDEwNC4yOC4yOC4zNCBPtWwTIp4-T40ZbjCdyCfeStS1-WkKW8w_WWEQubJpyQ1qcC50aWFyYXAub3JnCi9kbnMtcXVlcnk",
        "sdns://AgcAAAAAAAAAGVsyNjA2OjQ3MDA6MzA6OjY4MWM6MWQyMl0gT7VsEyKePk-NGW4wncgn3krUtflpClvMP1lhELmyackNanAudGlhcmFwLm9yZwovZG5zLXF1ZXJ5",
        "sdns://AgYAAAAAAAAAACA-GhoPbFPz6XpJLVcIS1uYBwWe4FerFQWHb9g_2j24OA9kb2gubGlicmVkbnMuZ3IKL2Rucy1xdWVyeQ",
        "sdns://AQcAAAAAAAAAETEzOS41OS41NS4xMzo0MzQzIMI_cHfgQzHFYUiS8m2ghR8Ij9nb88EQZXAYDlOhBGhmHzIuZG5zY3J5cHQtY2VydC5kbnMubXJrYXJhbi5kZXY",
        "sdns://AgcAAAAAAAAADjE4NS4xNTcuMjMzLjkyID4aGg9sU_PpekktVwhLW5gHBZ7gV6sVBYdv2D_aPbg4EGRvaC5uZXR3ZWF2ZXIudWsKL2Rucy1xdWVyeQ",
        "sdns://AgUAAAAAAAAACzUuMTgyLjIwOC4wID4aGg9sU_PpekktVwhLW5gHBZ7gV6sVBYdv2D_aPbg4DmRucy5uZXh0ZG5zLmlvDy9kbnNjcnlwdC1wcm94eQ",
        "sdns://AQcAAAAAAAAAEzE5NS4xMC4xOTUuMTk1OjUzNTMg8hbE05QkH0WdwNiGcxtcLvFewNj3USVp1A-VL0P77HIoMi5kbnNjcnlwdC1jZXJ0Lm9wZW5uaWMyLmV0aC1zZXJ2aWNlcy5kZQ",
        "sdns://AQYAAAAAAAAAEzE5NS4xMC4xOTUuMTk1OjUzNTMg8hbE05QkH0WdwNiGcxtcLvFewNj3USVp1A-VL0P77HIoMi5kbnNjcnlwdC1jZXJ0Lm9wZW5uaWMyLmV0aC1zZXJ2aWNlcy5kZQ",
        "sdns://AQYAAAAAAAAADTE0Mi40LjIwNC4xMTEgHBl5MxvoI8zPCJp5BpN-XDQQKlasf2Jw4EYlsu3bBOMfMi5kbnNjcnlwdC1jZXJ0Lm5zMy5jYS5sdWdncy5jbw",
        "sdns://AQYAAAAAAAAAIVsyNjA3OjUzMDA6MTIwOmE4YToxNDI6NDoyMDQ6MTExXSAcGXkzG-gjzM8ImnkGk35cNBAqVqx_YnDgRiWy7dsE4x8yLmRuc2NyeXB0LWNlcnQubnMzLmNhLmx1Z2dzLmNv",
        "sdns://AQYAAAAAAAAAEDE0Mi40LjIwNS40Nzo0NDMgvL-34FDBPaJCLACwsaya1kjFwmS8thcLiD1xishuugkfMi5kbnNjcnlwdC1jZXJ0Lm5zNC5jYS5sdWdncy5jbw",
        "sdns://AQYAAAAAAAAAJFsyNjA3OjUzMDA6MTIwOmE4YToxNDI6NDoyMDU6NDddOjQ0MyC8v7fgUME9okIsALCxrJrWSMXCZLy2FwuIPXGKyG66CR8yLmRuc2NyeXB0LWNlcnQubnM0LmNhLmx1Z2dzLmNv",
        "sdns://AQIAAAAAAAAAETUxLjM4LjgzLjE0MTo1MzUzIMwm9_oYw26P4JIVoDhJ_5kFDdNxX1ke4fEzL1V5bwEjFzIuZG5zY3J5cHQtY2VydC5vc3p4LmNv",
        "sdns://AgcAAAAAAAAAACA-GhoPbFPz6XpJLVcIS1uYBwWe4FerFQWHb9g_2j24OBBkb2gucG93ZXJkbnMub3JnAS8",
        "sdns://AQcAAAAAAAAADDQ1Ljc2LjExMy4zMSAIVGh4i6eKXqlF6o9Fg92cgD2WcDvKQJ7v_Wq4XrQsVhsyLmRuc2NyeXB0LWNlcnQuZG5zLnNlYnkuaW8",
        "sdns://AgcAAAAAAAAADDQ1Ljc2LjExMy4zMSA-GhoPbFPz6XpJLVcIS1uYBwWe4FerFQWHb9g_2j24OBBkb2guc2VieS5pbzo4NDQzCi9kbnMtcXVlcnk",
        "sdns://AQcAAAAAAAAAEjEzOS45OS4yMjIuNzI6ODQ0MyALBWhPDQvh2BqXksLUVtlS0e0tH-tW5XqtWfE73l7gZRsyLmRuc2NyeXB0LWNlcnQuZG5zLnNlYnkuaW8",
        "sdns://AgcAAAAAAAAADTEzOS45OS4yMjIuNzIgPhoaD2xT8-l6SS1XCEtbmAcFnuBXqxUFh2_YP9o9uDgRZG9oLTIuc2VieS5pbzo0NDMKL2Rucy1xdWVyeQ",
        "sdns://AQMAAAAAAAAAETEwNi41MS40NC41MTo0NDM0ING7Nd6ynAGtsN3sIyHYjkWg7pcKTGvRz4dOe0sbDZjcFjIuZG5zY3J5cHQtY2VydC5xYWcubWU",
        "sdns://AgcAAAAAAAAAACAoPxWWFWiOuUdTdn7SvYpzbNqr_iDmmJrktihy4wca5gxkbnMudHduaWMudHcKL2Rucy1xdWVyeQ",
        "sdns://AQMAAAAAAAAADDkuOS45Ljk6ODQ0MyBnyEe4yHWM0SAkVUO-dWdG3zTfHYTAC4xHA2jfgh2GPhkyLmRuc2NyeXB0LWNlcnQucXVhZDkubmV0",
        "sdns://AQMAAAAAAAAAEjE0OS4xMTIuMTEyLjk6ODQ0MyBnyEe4yHWM0SAkVUO-dWdG3zTfHYTAC4xHA2jfgh2GPhkyLmRuc2NyeXB0LWNlcnQucXVhZDkubmV0",
        "sdns://AQYAAAAAAAAADTkuOS45LjEwOjg0NDMgZ8hHuMh1jNEgJFVDvnVnRt803x2EwAuMRwNo34Idhj4ZMi5kbnNjcnlwdC1jZXJ0LnF1YWQ5Lm5ldA",
        "sdns://AQYAAAAAAAAAEzE0OS4xMTIuMTEyLjEwOjg0NDMgZ8hHuMh1jNEgJFVDvnVnRt803x2EwAuMRwNo34Idhj4ZMi5kbnNjcnlwdC1jZXJ0LnF1YWQ5Lm5ldA",
        "sdns://AQMAAAAAAAAAEVsyNjIwOmZlOjo5XTo4NDQzIGfIR7jIdYzRICRVQ751Z0bfNN8dhMALjEcDaN-CHYY-GTIuZG5zY3J5cHQtY2VydC5xdWFkOS5uZXQ",
        "sdns://AQMAAAAAAAAAFFsyNjIwOmZlOjpmZTo5XTo4NDQzIGfIR7jIdYzRICRVQ751Z0bfNN8dhMALjEcDaN-CHYY-GTIuZG5zY3J5cHQtY2VydC5xdWFkOS5uZXQ",
        "sdns://AQYAAAAAAAAAElsyNjIwOmZlOjoxMF06ODQ0MyBnyEe4yHWM0SAkVUO-dWdG3zTfHYTAC4xHA2jfgh2GPhkyLmRuc2NyeXB0LWNlcnQucXVhZDkubmV0",
        "sdns://AQYAAAAAAAAAFVsyNjIwOmZlOjpmZToxMF06ODQ0MyBnyEe4yHWM0SAkVUO-dWdG3zTfHYTAC4xHA2jfgh2GPhkyLmRuc2NyeXB0LWNlcnQucXVhZDkubmV0",
        "sdns://AgMAAAAAAAAABzkuOS45LjmAABJkbnM5LnF1YWQ5Lm5ldDo0NDMKL2Rucy1xdWVyeQ",
        "sdns://AgMAAAAAAAAADTE0OS4xMTIuMTEyLjmAABJkbnM5LnF1YWQ5Lm5ldDo0NDMKL2Rucy1xdWVyeQ",
        "sdns://AgYAAAAAAAAACDkuOS45LjEwgAASZG5zOS5xdWFkOS5uZXQ6NDQzCi9kbnMtcXVlcnk",
        "sdns://AgYAAAAAAAAADjE0OS4xMTIuMTEyLjEwgAASZG5zOS5xdWFkOS5uZXQ6NDQzCi9kbnMtcXVlcnk",
        "sdns://AgMAAAAAAAAADFsyNjIwOmZlOjo5XYAAEmRuczkucXVhZDkubmV0OjQ0MwovZG5zLXF1ZXJ5",
        "sdns://AgMAAAAAAAAAD1syNjIwOmZlOjpmZTo5XYAAEmRuczkucXVhZDkubmV0OjQ0MwovZG5zLXF1ZXJ5",
        "sdns://AgYAAAAAAAAADVsyNjIwOmZlOjoxMF2AABJkbnM5LnF1YWQ5Lm5ldDo0NDMKL2Rucy1xdWVyeQ",
        "sdns://AgYAAAAAAAAAEFsyNjIwOmZlOjpmZToxMF2AABJkbnM5LnF1YWQ5Lm5ldDo0NDMKL2Rucy1xdWVyeQ",
        "sdns://AQcAAAAAAAAAEjE3My44Mi4yMzIuMjMyOjg1MyCPlK_22Cu9WRVyKgl-CZp2GXezsRDWizG-BHIzChok4iAyLmRuc2NyeXB0LWNlcnQucXVhbGl0eW9sb2d5LmNvbQ",
        "sdns://AgUAAAAAAAAAACA-GhoPbFPz6XpJLVcIS1uYBwWe4FerFQWHb9g_2j24OBJlYS1kbnMucnVieWZpc2guY24KL2Rucy1xdWVyeQ",
        "sdns://AgUAAAAAAAAAACA-GhoPbFPz6XpJLVcIS1uYBwWe4FerFQWHb9g_2j24OBJ1dy1kbnMucnVieWZpc2guY24KL2Rucy1xdWVyeQ",
        "sdns://AQcAAAAAAAAADjIxMi40Ny4yMjguMTM2IOgBuE6mBr-wusDOQ0RbsV66ZLAvo8SqMa4QY2oHkDJNHzIuZG5zY3J5cHQtY2VydC5mci5kbnNjcnlwdC5vcmc",
        "sdns://AQcAAAAAAAAAEzE0Ni4xODUuMTY3LjQzOjUzNTMg9J8sc01itoYxntB-aRlDOy8ThfQe-8ovF21ZCy5FPoYcMi5kbnNjcnlwdC1jZXJ0LnNlY3VyZWRucy5ldQ",
        "sdns://AgcAAAAAAAAADjE0Ni4xODUuMTY3LjQzID4aGg9sU_PpekktVwhLW5gHBZ7gV6sVBYdv2D_aPbg4EGRvaC5zZWN1cmVkbnMuZXUKL2Rucy1xdWVyeQ",
        "sdns://AgcAAAAAAAAAHFsyYTAzOmIwYzA6MDoxMDEwOjplOWE6MzAwMV0gPhoaD2xT8-l6SS1XCEtbmAcFnuBXqxUFh2_YP9o9uDgQZG9oLnNlY3VyZWRucy5ldQovZG5zLXF1ZXJ5",
        "sdns://AQcAAAAAAAAAEzE2My4xNzIuMTgwLjEyNTo0NDMg32Jzv8dSGSqLWjm8DIWsP_lkRdc2RPZicoJdNVjxof8fMi5kbnNjcnlwdC1jZXJ0LnNmdy5zY2FsZXdheS1mcg",
        "sdns://AQcAAAAAAAAAFDE3OC4yMTYuMjAxLjIyMjoyMDUzICXE4YgpFUaXj5wrvbanr6QB7aBRBQhdUwPnGSjAZo8hHTIuZG5zY3J5cHQtY2VydC5zb2x0eXNpYWsuY29t",
        "sdns://AQcAAAAAAAAAETUxLjE1OC4xMDYuNDI6NDQzIPl5tBLA7XTAdkyZG0P642dOwlivYit19hW0I1yxtPMBHDIuZG5zY3J5cHQtY2VydC5pdC5zdWFtaS5jb20",
        "sdns://AQcAAAAAAAAADTEwNy4xNzAuNTcuMzQg6YXxGK1OPMZf8iUgGJDG9Vi3W1pS9WsXz-rBAFyLm6olMi5kbnNjcnlwdC1jZXJ0LmRuc2NyeXB0LnZlbnRyaWNsZS51cw",
        "sdns://AgUAAAAAAAAAAKAx_Wo8PCx8I5Gkl1qfoqes0mp4xMrk1W5GIynLLGg2syANc6YU7vdZbPWjRzP3Ta8sz-Tfe0pABpv0PEPkKCZBdw9kb2gueGZpbml0eS5jb20KL2Rucy1xdWVyeQ",
        "sdns://AQQAAAAAAAAAEDc3Ljg4LjguNzg6MTUzNTMg04TAccn3RmKvKszVe13MlxTUB7atNgHhrtwG1W1JYyciMi5kbnNjcnlwdC1jZXJ0LmJyb3dzZXIueWFuZGV4Lm5ldA",
        "sdns://AQYAAAAAAAAAETE1MS44MC4yMjIuNzk6NDQzIO4Y9lZnORlvodxu39dnm6mFruwTRnlmovbEga4Fyw3TIDIuZG5zY3J5cHQtY2VydC5vcGVubmljLmkycGQueHl6",
        "sdns://AQIAAAAAAAAADDc4LjQ3LjY0LjE2MSATJeLOABXNSYcSJIoqR5_iUYz87Y4OecMLB84aEAKPrRBkbnNmb3JmYW1pbHkuY29t",
        "sdns://AQIAAAAAAAAAF1syYTAxOjRmODoxYzE3OjRkZjg6OjFdIGN4CrSY4fb2hK8voFJL3GKiM7xQNwkKGH4b0k7LmMPxEGRuc2ZvcmZhbWlseS5jb20"];

    for stamp_1 in stamps.iter() {
        match DnsStamp::decode(stamp_1) {
            Ok(dns_stamp_1) => {
                match dns_stamp_1.encode() {
                    Ok(stamp_2) => {
                        match DnsStamp::decode(&stamp_2) {
                            Ok(dns_stamp_2) => {
                                if dns_stamp_1 != dns_stamp_2 {
                                    panic!("Not equal: {} {}", stamp_1, stamp_2);
                                }
                            }
                            Err(e) => {
                                panic!("Decode 2: {:?}: {} {}", e, stamp_1, stamp_2);
                            }
                        }
                    }
                    Err(e) => {
                        panic!("Encode 1: {:?}: {}", e, stamp_1);
                    }
                }
            }
            Err(e) => {
                panic!("Decode 1: {:?}: {}", e, stamp_1);
            }
        }
    }
}