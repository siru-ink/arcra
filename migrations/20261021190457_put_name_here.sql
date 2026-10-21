INSERT INTO currencies (id, name, descriptor, symbol) VALUES
(1, 'USD', 'united states dollar', '$'),
(2, 'EUR', 'euro', '€'),
(3, 'CNY', 'renminbi', '¥'),
(4, 'INR', 'indian rupee', '₹');

INSERT INTO accounts (name, currency, balance, account_type, category) VALUES
('USD_exchange', 1, 0, 6, 1),
('EUR_exchange', 1, 0, 6, 1),
('CNY_exchange', 1, 0, 6, 1),
('INR_exchange', 1, 0, 6, 1);
