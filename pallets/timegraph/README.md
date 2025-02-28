# Timegraph Pallet

Timegraph pallet manages the funds received by other timegraph users to timegraph server account which are fees for querying data.
Timegraph user can send funds using `deposit` extrinsic and withdraw funds using `withdraw` extrinsic.

## Storage:
### NextDepositSequence
`Counter for previous number of deposits made`

### NextWithdrawalSequence
`Counter for previous number of withdrawal made`

## Events:
### Deposit(T::AccountId, T::AccountId, BalanceOf<T>, u64),
`Amount deposited from timegraph user to timegaph account`

### Withdrawal(T::AccountId, T::AccountId, BalanceOf<T>, u64),
`Amount refunded from timegraph amount to timegaph user`

## Extrinsics:
### deposit(T::AccountId,BalanceOf<T>)
### Origin:
`Timegraph user`
### Purpose:
`The extrinsic from timegraph user to deposit funds into the timegraph account`

### withdraw(T::AccountId,BalanceOf<T>,u64)
### Origin:
`Timegraph account`
### Purpose:
`The extrinsic from timegraph user to deposit funds into the timegraph account`
