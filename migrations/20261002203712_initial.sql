CREATE TABLE account_types (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL,
    balance_type TEXT NOT NULL
);

INSERT INTO account_types (name, balance_type) VALUES
(asset, debit),
(expense, debit),
(liability, credit),
(equity, credit),
(revenue, credit);

CREATE TABLE currencies (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL,
    descriptor TEXT UNIQUE NOT NULL,
    symbol TEXT NOT NULL
);

INSERT INTO currencies (name, descriptor, symbol) VALUES
('EUR', 'euro', '€'),
('USD', 'united states dollar', '$'),
('CNY', 'renminbi', '¥');

CREATE TABLE accounts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL,
    currency INTEGER REFERENCES currencies(id) ON DELETE RESTRICT,
    balance INTEGER NOT NULL,
    account_type INTEGER NOT NULL REFERENCES account_types(id) ON DELETE RESTRICT
);

CREATE TABLE records (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    credit_account INTEGER NOT NULL REFERENCES accounts(id) ON DELETE RESTRICT,
    debit_account INTEGER NOT NULL REFERENCES accounts(id) ON DELETE RESTRICT,
    amount INTEGER NOT NULL,
    currency INTEGER REFERENCES currencies(id) ON DELETE RESTRICT
);
