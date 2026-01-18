// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/**
 * @title IReMLVerifier
 * @notice Interface for Tesserax ZK-Coprocessor precompiles
 * @dev These are EVM precompiles that allow smart contracts to:
 *      1. Verify STARK proof commitment structures
 *      2. Check if a request has been verified via Re-ML
 *      3. Get information about verified batches
 *
 * Precompile Addresses:
 * - 0x20: VerifyStarkCommitment (50,000 base gas + 100/byte)
 * - 0x21: IsRequestVerified (10,000 gas)
 * - 0x22: GetBatchInfo (15,000 gas)
 */

/**
 * @notice Interface for checking request verification status
 */
interface IReMLVerifier {
    /**
     * @notice Check if a request ID has been verified via Re-ML STARK proof
     * @param requestId The request ID to check (little-endian u64)
     * @return verified True if the request has been verified
     *
     * @dev Usage:
     * (bool success, bytes memory data) = address(0x21).staticcall(abi.encodePacked(requestId));
     * bool verified = success && abi.decode(data, (bool));
     */
    function isRequestVerified(
        uint64 requestId
    ) external view returns (bool verified);

    /**
     * @notice Get information about a verified batch
     * @param batchId The batch ID to query
     * @return requestsRoot The merkle root of request IDs in this batch (32 bytes)
     * @return signatureCount Number of signatures in the batch
     * @return verifiedAtBlock Block number when batch was verified
     *
     * @dev If batch is not found, returns all zeros
     */
    function getBatchInfo(
        uint64 batchId
    )
        external
        view
        returns (
            bytes32 requestsRoot,
            uint32 signatureCount,
            uint64 verifiedAtBlock
        );
}

/**
 * @title ReMLVerifierLib
 * @notice Library for interacting with Tesserax ZK-Coprocessor precompiles
 * @dev This library provides helper functions for calling the precompiles
 */
library ReMLVerifierLib {
    // Precompile addresses
    address constant VERIFY_STARK_COMMITMENT = address(0x20);
    address constant IS_REQUEST_VERIFIED = address(0x21);
    address constant GET_BATCH_INFO = address(0x22);

    /**
     * @notice Check if a STARK proof commitment is structurally valid
     * @param vkeyHash The verification key hash (32 bytes)
     * @param publicCommitment The public values commitment (32 bytes)
     * @param proofData The proof data (variable length)
     * @return valid True if the commitment structure is valid
     *
     * @dev This is a lightweight check. Full verification happens on-chain
     * via pallet-reml-verifier when submitting proofs.
     */
    function verifyStarkCommitment(
        bytes32 vkeyHash,
        bytes32 publicCommitment,
        bytes calldata proofData
    ) internal view returns (bool valid) {
        bytes memory input = abi.encodePacked(
            vkeyHash,
            publicCommitment,
            proofData
        );

        (bool success, bytes memory result) = VERIFY_STARK_COMMITMENT
            .staticcall(input);

        if (!success || result.length < 32) {
            return false;
        }

        // Check last byte for true/false
        return result[31] != 0;
    }

    /**
     * @notice Check if a specific request ID has been verified via Re-ML
     * @param requestId The request ID to check
     * @return verified True if verified
     */
    function isRequestVerified(
        uint64 requestId
    ) internal view returns (bool verified) {
        bytes memory input = abi.encodePacked(requestId);

        (bool success, bytes memory result) = IS_REQUEST_VERIFIED.staticcall(
            input
        );

        if (!success || result.length < 32) {
            return false;
        }

        return result[31] != 0;
    }

    /**
     * @notice Require that a request has been verified, revert otherwise
     * @param requestId The request ID that must be verified
     */
    function requireVerified(uint64 requestId) internal view {
        require(isRequestVerified(requestId), "ReML: Request not verified");
    }

    /**
     * @notice Get batch information for a verified batch
     * @param batchId The batch ID
     * @return found True if batch exists
     * @return requestsRoot Merkle root of request IDs
     * @return signatureCount Number of signatures
     * @return verifiedAtBlock Block number of verification
     */
    function getBatchInfo(
        uint64 batchId
    )
        internal
        view
        returns (
            bool found,
            bytes32 requestsRoot,
            uint32 signatureCount,
            uint64 verifiedAtBlock
        )
    {
        bytes memory input = abi.encodePacked(batchId);

        (bool success, bytes memory result) = GET_BATCH_INFO.staticcall(input);

        if (!success || result.length < 64) {
            return (false, bytes32(0), 0, 0);
        }

        // Parse result
        // bytes[0..32]: requests root
        // bytes[32..36]: signature count (big-endian u32)
        // bytes[36..44]: block number (big-endian u64)

        assembly {
            requestsRoot := mload(add(result, 32))
        }

        // Parse signature count (big-endian u32)
        signatureCount =
            (uint32(uint8(result[32])) << 24) |
            (uint32(uint8(result[33])) << 16) |
            (uint32(uint8(result[34])) << 8) |
            uint32(uint8(result[35]));

        // Parse block number (big-endian u64)
        verifiedAtBlock =
            (uint64(uint8(result[36])) << 56) |
            (uint64(uint8(result[37])) << 48) |
            (uint64(uint8(result[38])) << 40) |
            (uint64(uint8(result[39])) << 32) |
            (uint64(uint8(result[40])) << 24) |
            (uint64(uint8(result[41])) << 16) |
            (uint64(uint8(result[42])) << 8) |
            uint64(uint8(result[43]));

        // Check if this is a valid batch (non-zero root)
        found = requestsRoot != bytes32(0);
    }
}

/**
 * @title QuantumSafeBase
 * @notice Base contract for quantum-safe applications
 * @dev Inherit from this contract to add Re-ML verification to your dApp
 */
abstract contract QuantumSafeBase {
    using ReMLVerifierLib for uint64;

    /**
     * @notice Modifier to require Re-ML verification for a request
     * @param requestId The request ID that must be verified
     */
    modifier requiresQuantumProof(uint64 requestId) {
        require(
            ReMLVerifierLib.isRequestVerified(requestId),
            "QuantumSafe: Quantum proof required"
        );
        _;
    }

    /**
     * @notice Check if a request has been quantum-verified
     */
    function isQuantumVerified(uint64 requestId) public view returns (bool) {
        return ReMLVerifierLib.isRequestVerified(requestId);
    }
}

/**
 * @title QuantumVaultEVM
 * @notice Example: EVM wrapper for Quantum Vault functionality
 * @dev Demonstrates how to integrate Re-ML verification with smart contracts
 */
contract QuantumVaultEVM is QuantumSafeBase {
    // Events
    event TransferQueued(
        uint64 indexed requestId,
        address indexed from,
        address indexed to,
        uint256 amount
    );
    event TransferExecuted(uint64 indexed requestId);

    // Storage
    struct PendingTransfer {
        address from;
        address to;
        uint256 amount;
        bool executed;
    }

    mapping(uint64 => PendingTransfer) public pendingTransfers;
    uint64 public nextRequestId = 1;

    /**
     * @notice Queue a quantum-safe transfer
     * @param to Recipient address
     * @param amount Amount to transfer
     * @return requestId The request ID for this transfer
     *
     * @dev After queueing, the transfer must be verified via Re-ML
     * before it can be executed.
     */
    function queueTransfer(
        address to,
        uint256 amount
    ) external returns (uint64 requestId) {
        requestId = nextRequestId++;

        pendingTransfers[requestId] = PendingTransfer({
            from: msg.sender,
            to: to,
            amount: amount,
            executed: false
        });

        emit TransferQueued(requestId, msg.sender, to, amount);
    }

    /**
     * @notice Execute a quantum-verified transfer
     * @param requestId The request ID of the transfer
     *
     * @dev This can only succeed after the request has been verified
     * via the Re-ML off-chain prover network.
     */
    function executeTransfer(
        uint64 requestId
    ) external requiresQuantumProof(requestId) {
        PendingTransfer storage transfer = pendingTransfers[requestId];

        require(!transfer.executed, "Already executed");
        require(transfer.from != address(0), "Transfer not found");

        transfer.executed = true;

        // Execute the transfer logic here
        // (In a real implementation, this would transfer tokens)

        emit TransferExecuted(requestId);
    }

    /**
     * @notice Check if a transfer is ready to execute
     */
    function isTransferReady(uint64 requestId) external view returns (bool) {
        PendingTransfer storage transfer = pendingTransfers[requestId];
        return
            !transfer.executed &&
            transfer.from != address(0) &&
            ReMLVerifierLib.isRequestVerified(requestId);
    }
}
