CREATE TABLE currencies (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL,
    descriptor TEXT UNIQUE NOT NULL,
    symbol TEXT NOT NULL
);

-- INSERT INTO currencies (name, descriptor, symbol) VALUES
-- ('EUR', 'euro', '€'),
-- ('USD', 'united states dollar', '$'),
-- ('CNY', 'renminbi', '¥');

CREATE TABLE categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL,
    description TEXT UNIQUE NOT NULL
);

INSERT INTO categories (id, name, description) VALUES
(1, 'none', 'default category for non expense accounts');

CREATE TABLE accounts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL,
    currency INTEGER NOT NULL REFERENCES currencies(id) ON DELETE RESTRICT,
    balance INTEGER NOT NULL,
    account_type INTEGER NOT NULL,
    category INTEGER REFERENCES categories(id) ON DELETE RESTRICT
);

CREATE TABLE transactions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    description TEXT NOT NULL,
    time TEXT NOT NULL,
    credit_account INTEGER NOT NULL REFERENCES accounts(id) ON DELETE RESTRICT,
    credit_amount INTEGER NOT NULL,
    debit_account INTEGER NOT NULL REFERENCES accounts(id) ON DELETE RESTRICT,
    debit_amount INTEGER NOT NULL
);

CREATE TABLE sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    valid_until TEXT NOT NULL
);
