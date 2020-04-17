pragma solidity ^0.6.4;
pragma experimental ABIEncoderV2;

import '../stark_verifier.sol';
import '../proof_types.sol';


interface ConstraintSystem {
    // The function should return a constraint paramters struct based on the public input.
    function initalize_system(bytes calldata public_input)
        external
        view
        returns (ProofTypes.ProofParameters memory, PublicCoin.Coin memory);

    // This function should take all of the relevent function information and then return two things
    // (1) the evaulation of the constraints on the oods point and
    // (2) the calculation of the points on the polynomial which is commited too for fri
    function constraint_calculations(
        ProofTypes.StarkProof calldata proof,
        ProofTypes.ProofParameters calldata params,
        uint64[] calldata queries,
        bytes32 oods_point,
        bytes32[] calldata constraint_coeffiencts
    ) external view returns (bytes32[] memory, bytes32);
}
