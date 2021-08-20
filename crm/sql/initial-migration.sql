select setval(pg_get_serial_sequence('account', 'account_id'), 100);

INSERT INTO public.account (account_id, name, password_hash, roles, policies, created, allow_login)
VALUES (1, 'admin', '$2b$10$Fgd3GBic2Jqcx2QLXUlsteTDST0ycXjnuMGzKbA4tTmgqr17PkgKC', '[1]', '[]',
        '2020-01-01 00:00:00.000000', true);
INSERT INTO public.account (account_id, name, password_hash, roles, policies, created, allow_login)
VALUES (2, 'maker', '$2b$10$Y0gJZ83Ccc79CNsglYXGxOKAODlsSBYKOkNFT8oMF8ZV6ILqtVitK', '[2]', '[]',
        '2020-01-01 00:00:00.000000', true);