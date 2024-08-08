// migrations/2_deploy_contracts.js
const Dependency = artifacts.require("Dependency");
const MainContract = artifacts.require("MainContract");

module.exports = async function (deployer) {
  await deployer.deploy(Dependency);
  const dependency = await Dependency.deployed();

  await deployer.deploy(MainContract, dependency.address);
};