import { task } from "hardhat/config";

// npx hardhat get-greeting  --contract 0x94766f7a4301f3A4593C2ea7f574155c9648Ce88  

task("getscore", "Fetches the game score")
  .addParam("contract", "The address of the deployed getscore contract")
  .setAction(async ({ contract }, hre) => {
    const { ethers } = hre;
    
    const getBlockNumber = await ethers.provider.getBlockNumber()
    console.log("Block Number:", getBlockNumber);

    const GreetingWithWorld = await ethers.getContractAt(
      "IFluentGreeting",
      contract
    );

    const result = await GreetingWithWorld.getResult();
    // Convert the string result for display
    console.log("Game result:", result.toString());

  
  });


  