import { patract, network } from "redspot";

const { getContractFactory } = patract;
const { createSigner, keyring, api } = network;

async function run() {
  await api.isReady;

  // The redspot signer supports passing in an address. If you want to use  substrate uri, you can do it like this:
  // const signer = createSigner(keyring.createFromUri("bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice"));
  // Or get the configured account from redspot config:
  // const signer = (await getSigners())[0]
  const signer = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"; // Alice Address

  const erc20Factory = await getContractFactory("erc20", signer);
  const erc20MinableFactory = await getContractFactory("miner_erc20", signer);
  const allFactory = await getContractFactory("all", signer);

  const balance = await api.query.system.account(signer);

  console.log("Balance: ", balance.toHuman());

  // The `deploy` method will attempt to deploy a new contract.
  // The `deployed` method will first find out if the same contract already exists based on the parameters.
  // If the contract exists, it will be returned, otherwise a new contract will be created.
  const erc20 = await erc20Factory.deploy("new", "1000000", {
    gasLimit: "400000000000",
    value: "1000 UNIT",
  });

  console.log("");
  console.log(
    "Deploy successfully. The contract erc20 address: ",
    erc20.address.toString()
  );

  const erc20Minable = await erc20MinableFactory.deploy("new", "2000000", {
    gasLimit: "400000000000",
    value: "1000 UNIT",
  });
  console.log("");
  console.log(
    "Deploy successfully. The contract erc20Minable address: ",
    erc20Minable.address.toString()
  );

  const all = await allFactory.deploy("new", erc20.address, erc20Minable.address, {
    gasLimit: "400000000000",
    value: "1000 UNIT",
  });
  console.log("");
  console.log(
    "Deploy successfully. The contract all address: ",
    all.address.toString()
  );

  const someAddr = "5FbpwTP4usJ7dCFvtwzpew6NfXhtkvZH1jY4h6UfLztyD3Ng";

  await erc20.tx.transfer(someAddr, 123);
  console.log("transfer 1")
  await erc20Minable.tx.transfer(someAddr, 456);
  console.log("transfer 2")

  console.log("first");
  try {
    await all.tx.transfer1(someAddr, 123);
  } catch {
    console.log("ignore this error");
  }
  

  console.log("second");
  const result = await all.query.balanceOf(someAddr);
  console.log(result.output?.toString());

  
  console.log("third");
  await all.tx.mine(someAddr, 999);
  const result2 = await erc20Minable.query.balanceOf(someAddr);
  console.log(result2.output?.toString());

  api.disconnect();
}

run().catch((err) => {
  console.log(err);
});
