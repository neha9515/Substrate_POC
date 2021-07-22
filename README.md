# Bookshop
The project Bookshop is developed in the substrate that helps to keep track of all the book records and customer details.

## Installation
You will need to do some setup to prepare your system for substrate development. Simply go to [substrate.dev](https://substrate.dev/) and follow the installation instruction.

## Add a Pallet
This project contains two pallets, a store pallet to maintain book details such as Book ISBN no, Book name, Book author name, Book price and a customer pallet to maintain customer details such as Customer ID, Customer name, Customer contact no, Book name which is customer wants to buy, Book author name, Number of copies and Book price.

## Run Locally
To add your pallet, Substrate-Node-Template/pallets

#### Clone the pallet

 > git clone -b v3.0.0 --depth 1 https://github.com/substrate-developer-hub/substrate-pallet-template store-pallet
  
#### Go to the pallet

 > cd store-pallet
  
#### To add more dependencies just go through [substrate-pallet-template](https://github.com/substrate-developer-hub/substrate-pallet-template/blob/master/README.md)

Once you added all dependencies run the below commands in the root directory

 > cargo build --release 
  
After the build succeeds, you can start the node

 > ./target/release/node-template --dev --tmp
 
Once your node is running, you will notice that blocks are being produced. At that time go to the [polkadot.js](https://polkadot.js.org/apps/#/explorer)

#### Click on, Developer->extrinsics->store

The fields can be filled like this,
![store1](https://user-images.githubusercontent.com/85206495/126626548-cb755579-8be7-47aa-b849-54b6ff2a46e1.png)

You've now successfully inserted the book details.

#### Click on, Network->explorer
![store2](https://user-images.githubusercontent.com/85206495/126626612-f653799b-f79a-42e6-aff2-79695f36be8d.png)

- The same process will be followed for the customer pallet.
