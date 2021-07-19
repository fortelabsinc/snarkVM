// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use snarkvm_dpc::{testnet1::parameters::*, Account, AccountScheme, DPCScheme};

use rand::{CryptoRng, Rng};

pub fn setup_or_load_parameters<R: Rng + CryptoRng>(verify_only: bool, rng: &mut R) -> Testnet1DPC {
    match Testnet1DPC::load(verify_only) {
        Ok(dpc) => dpc,
        Err(err) => {
            println!("error - {}, re-running parameter Setup", err);
            Testnet1DPC::setup(rng).expect("DPC setup failed")
        }
    }
}

pub fn generate_test_accounts<R: Rng + CryptoRng>(rng: &mut R) -> [Account<Testnet1Parameters>; 3] {
    let genesis_account = Account::new(rng).unwrap();
    let account_1 = Account::new(rng).unwrap();
    let account_2 = Account::new(rng).unwrap();

    [genesis_account, account_1, account_2]
}
