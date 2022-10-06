import "art-gobblers/ArtGobblers.sol";
import "art-gobblers/Goo.sol";

contract MockGobbler is ArtGobblers
{
    constructor() ArtGobblers(
        bytes32(0), // merkleRoot
        block.timestamp + 1 days, // mintStart
        Goo(address(0)), // goo
        address(0), // team
        address(0), // community
        address(0), // vrfCoordinator
        address(0), // linkToken
        bytes32(0), // chainlinkKeyHash
        0, // chainlinkFee
        "", // baseuri
        "" // unrevealedUri
    )
    {
    }

    // A helper function to help with testing
    // We want to test revealing all the gobblers to understand the randomness
    function setToBeRevealed(uint64 randomseed) external {
        gobblerRevealsData.randomSeed = randomseed;
        // TODO what should be the correct number?
        gobblerRevealsData.toBeRevealed = 10000;
    }

    function getAllGobblerData() external view returns (uint[9990] memory idxs) {
        unchecked {
            for(uint i = 1; i <= 9990; i++) {
                idxs[i - 1] = getGobblerData[i].idx;
            }
        }
    }
}
