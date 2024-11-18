import { task } from "hardhat/config";

// npx hardhat get-greeting  --contract 0x94766f7a4301f3A4593C2ea7f574155c9648Ce88  

task("startgame", "Fetches the greeting from the deployed GreetingWithWorld contract")
  .addParam("contract", "The address of the deployed GreetingWithWorld contract")
  .setAction(async ({ contract }, hre) => {
    const { ethers } = hre;
    
    const getBlockNumber = await ethers.provider.getBlockNumber()
    console.log("Block Number:", getBlockNumber);

    const GreetingWithWorld = await ethers.getContractAt(
      "IFluentGreeting",
      contract
    );

    const greeting = await GreetingWithWorld.startGame();
    // Convert the BigInt to a string for display
    console.log("Start game output:", greeting.toString());

  
  });


  