#![cfg(all(feature = "ef_tests", feature = "gnosis_tests"))]
//! Gnosis-preset reference tests.
//!
//! Auto-derived from `tests.rs`: the preset-dependent EF handlers run against the Gnosis
//! preset (`GnosisEthSpec`), reading vectors from `consensus-spec-tests/tests/gnosis/`.
//! Built only with the `gnosis_tests` feature so it runs as its own CI job.


use ef_tests::*;
use typenum::Unsigned;
use types::*;

// Check that the hand-computed multiplications on EthSpec are correctly computed.
// This test lives here because one is most likely to muck these up during a spec update.
fn check_typenum_values<E: EthSpec>() {
    assert_eq!(
        E::MaxPendingAttestations::to_u64(),
        E::MaxAttestations::to_u64() * E::SlotsPerEpoch::to_u64()
    );
    assert_eq!(
        E::SlotsPerEth1VotingPeriod::to_u64(),
        E::EpochsPerEth1VotingPeriod::to_u64() * E::SlotsPerEpoch::to_u64()
    );
    assert_eq!(
        E::MaxValidatorsPerSlot::to_u64(),
        E::MaxCommitteesPerSlot::to_u64() * E::MaxValidatorsPerCommittee::to_u64()
    );
}

#[test]
fn derived_typenum_values() {
    check_typenum_values::<GnosisEthSpec>();
}

#[test]
fn shuffling() {
    ShufflingHandler::<GnosisEthSpec>::default().run();
}

#[test]
fn operations_deposit() {
    OperationsHandler::<GnosisEthSpec, Deposit>::default().run();
}

#[test]
fn operations_exit() {
    OperationsHandler::<GnosisEthSpec, SignedVoluntaryExit>::default().run();
}

#[test]
fn operations_proposer_slashing() {
    OperationsHandler::<GnosisEthSpec, ProposerSlashing>::default().run();
}

#[test]
fn operations_attester_slashing() {
    OperationsHandler::<GnosisEthSpec, AttesterSlashing<_>>::default().run();
}

#[test]
fn operations_attestation() {
    OperationsHandler::<GnosisEthSpec, Attestation<_>>::default().run();
}

#[test]
fn operations_block_header() {
    OperationsHandler::<GnosisEthSpec, BeaconBlock<_>>::default().run();
}

#[test]
fn operations_sync_aggregate() {
    OperationsHandler::<GnosisEthSpec, SyncAggregate<_>>::default().run();
}

#[test]
fn operations_execution_payload_full() {
    OperationsHandler::<GnosisEthSpec, BeaconBlockBody<_, FullPayload<_>>>::default().run();
}

#[test]
fn operations_execution_payload_blinded() {
    OperationsHandler::<GnosisEthSpec, BeaconBlockBody<_, BlindedPayload<_>>>::default().run();
}

#[test]
fn operations_withdrawals() {
    OperationsHandler::<GnosisEthSpec, WithdrawalsPayload<_>>::default().run();
}

#[test]
fn operations_withdrawal_reqeusts() {
    OperationsHandler::<GnosisEthSpec, WithdrawalRequest>::default().run();
}

#[test]
#[cfg(not(feature = "fake_crypto"))]
fn operations_deposit_requests() {
    OperationsHandler::<GnosisEthSpec, DepositRequest>::default().run();
}

#[test]
fn operations_consolidations() {
    OperationsHandler::<GnosisEthSpec, ConsolidationRequest>::default().run();
}

#[test]
fn operations_bls_to_execution_change() {
    OperationsHandler::<GnosisEthSpec, SignedBlsToExecutionChange>::default().run();
}

#[test]
fn sanity_blocks() {
    SanityBlocksHandler::<GnosisEthSpec>::default().run();
}

#[test]
fn sanity_slots() {
    SanitySlotsHandler::<GnosisEthSpec>::default().run();
}

#[test]
fn random() {
    RandomHandler::<GnosisEthSpec>::default().run();
}

#[test]
#[cfg(not(feature = "fake_crypto"))]
fn bls_aggregate() {
    BlsAggregateSigsHandler::default().run();
}

#[test]
#[cfg(not(feature = "fake_crypto"))]
fn bls_sign() {
    BlsSignMsgHandler::default().run();
}

#[test]
#[cfg(not(feature = "fake_crypto"))]
fn bls_verify() {
    BlsVerifyMsgHandler::default().run();
}

#[test]
#[cfg(not(feature = "fake_crypto"))]
fn bls_batch_verify() {
    BlsBatchVerifyHandler::default().run();
}

#[test]
#[cfg(not(feature = "fake_crypto"))]
fn bls_aggregate_verify() {
    BlsAggregateVerifyHandler::default().run();
}

#[test]
#[cfg(not(feature = "fake_crypto"))]
fn bls_fast_aggregate_verify() {
    BlsFastAggregateVerifyHandler::default().run();
}

#[test]
#[cfg(not(feature = "fake_crypto"))]
fn bls_eth_aggregate_pubkeys() {
    BlsEthAggregatePubkeysHandler::default().run();
}

#[test]
#[cfg(not(feature = "fake_crypto"))]
fn bls_eth_fast_aggregate_verify() {
    BlsEthFastAggregateVerifyHandler::default().run();
}

/// As for `ssz_static_test_no_run` (below), but also executes the function as a test.
#[cfg(feature = "fake_crypto")]
macro_rules! ssz_static_test {
    ($($args:tt)*) => {
        ssz_static_test_no_run!(#[test] $($args)*);
    };
}

/// Generate a function to run the SSZ static tests for a type.
///
/// Quite complex in order to support an optional #[test] attrib, generics, and the two EthSpecs.
#[cfg(feature = "fake_crypto")]
macro_rules! ssz_static_test_no_run {
    // Top-level
    ($(#[$test:meta])? $test_name:ident, $typ:ident$(<$generics:tt>)?) => {
        ssz_static_test_no_run!($(#[$test])? $test_name, SszStaticHandler, $typ$(<$generics>)?);
    };
    // Generic
    ($(#[$test:meta])? $test_name:ident, $handler:ident, $typ:ident<_>) => {
        ssz_static_test_no_run!(
            $(#[$test])?
            $test_name,
            $handler,
            {
                ($typ<GnosisEthSpec>, GnosisEthSpec)
            }
        );
    };
    // Non-generic
    ($(#[$test:meta])? $test_name:ident, $handler:ident, $typ:ident) => {
        ssz_static_test_no_run!(
            $(#[$test])?
            $test_name,
            $handler,
            {
                ($typ, GnosisEthSpec)
            }
        );
    };
    // Base case
    ($(#[$test:meta])? $test_name:ident, $handler:ident, { $(($($typ:ty),+)),+ }) => {
        $(#[$test])?
        fn $test_name() {
            $(
                $handler::<$($typ),+>::default().run();
            )+
        }
    };
}

#[cfg(feature = "fake_crypto")]
mod ssz_static {
    use ef_tests::{Handler, SszStaticHandler, SszStaticTHCHandler, SszStaticWithSpecHandler};
    use types::state::HistoricalSummary;
    use types::{
        AttesterSlashingBase, AttesterSlashingElectra, ConsolidationRequest, DataColumnSidecarFulu,
        DataColumnSidecarGloas, DepositRequest, LightClientBootstrapAltair, PendingDeposit,
        PendingPartialWithdrawal, WithdrawalRequest, *,
    };

    ssz_static_test!(attestation_data, AttestationData);
    ssz_static_test!(beacon_block, SszStaticWithSpecHandler, BeaconBlock<_>);
    ssz_static_test!(beacon_block_header, BeaconBlockHeader);
    ssz_static_test!(beacon_state, SszStaticTHCHandler, BeaconState<_>);
    ssz_static_test!(checkpoint, Checkpoint);
    ssz_static_test!(deposit, Deposit);
    ssz_static_test!(deposit_data, DepositData);
    ssz_static_test!(deposit_message, DepositMessage);
    // NOTE: Eth1Block intentionally omitted, see: https://github.com/sigp/lighthouse/issues/1835
    ssz_static_test!(eth1_data, Eth1Data);
    ssz_static_test!(fork, Fork);
    ssz_static_test!(fork_data, ForkData);
    ssz_static_test!(historical_batch, HistoricalBatch<_>);
    ssz_static_test!(pending_attestation, PendingAttestation<_>);
    ssz_static_test!(proposer_slashing, ProposerSlashing);
    ssz_static_test!(
        signed_beacon_block,
        SszStaticWithSpecHandler,
        SignedBeaconBlock<_>
    );
    ssz_static_test!(signed_beacon_block_header, SignedBeaconBlockHeader);
    ssz_static_test!(signed_voluntary_exit, SignedVoluntaryExit);
    ssz_static_test!(signing_data, SigningData);
    ssz_static_test!(validator, Validator);
    ssz_static_test!(voluntary_exit, VoluntaryExit);

    #[test]
    fn attestation() {
        SszStaticHandler::<AttestationBase<GnosisEthSpec>, GnosisEthSpec>::pre_electra().run();
        SszStaticHandler::<AttestationElectra<GnosisEthSpec>, GnosisEthSpec>::electra_and_later()
            .run();
    }

    #[test]
    fn single_attestation() {
        SszStaticHandler::<SingleAttestation, GnosisEthSpec>::electra_and_later().run();
    }

    #[test]
    fn attester_slashing() {
        SszStaticHandler::<AttesterSlashingBase<GnosisEthSpec>, GnosisEthSpec>::pre_electra()
            .run();
        SszStaticHandler::<AttesterSlashingElectra<GnosisEthSpec>, GnosisEthSpec>::electra_and_later()
            .run();
    }

    #[test]
    fn indexed_attestation() {
        SszStaticHandler::<IndexedAttestationBase<GnosisEthSpec>, GnosisEthSpec>::pre_electra()
            .run();
        SszStaticHandler::<IndexedAttestationElectra<GnosisEthSpec>, GnosisEthSpec>::electra_and_later()
            .run();
    }

    #[test]
    fn signed_aggregate_and_proof() {
        SszStaticHandler::<SignedAggregateAndProofBase<GnosisEthSpec>, GnosisEthSpec>::pre_electra(
        )
        .run();
        SszStaticHandler::<SignedAggregateAndProofElectra<GnosisEthSpec>, GnosisEthSpec>::electra_and_later(
        )
        .run();
    }

    #[test]
    fn aggregate_and_proof() {
        SszStaticHandler::<AggregateAndProofBase<GnosisEthSpec>, GnosisEthSpec>::pre_electra()
            .run();
        SszStaticHandler::<AggregateAndProofElectra<GnosisEthSpec>, GnosisEthSpec>::electra_and_later(
        )
        .run();
    }

    // BeaconBlockBody has no internal indicator of which fork it is for, so we test it separately.
    #[test]
    fn beacon_block_body() {
        SszStaticHandler::<BeaconBlockBodyBase<GnosisEthSpec>, GnosisEthSpec>::base_only().run();
        SszStaticHandler::<BeaconBlockBodyAltair<GnosisEthSpec>, GnosisEthSpec>::altair_only()
            .run();
        SszStaticHandler::<BeaconBlockBodyBellatrix<GnosisEthSpec>, GnosisEthSpec>::bellatrix_only()
            .run();
        SszStaticHandler::<BeaconBlockBodyCapella<GnosisEthSpec>, GnosisEthSpec>::capella_only()
            .run();
        SszStaticHandler::<BeaconBlockBodyDeneb<GnosisEthSpec>, GnosisEthSpec>::deneb_only()
            .run();
        SszStaticHandler::<BeaconBlockBodyElectra<GnosisEthSpec>, GnosisEthSpec>::electra_only()
            .run();
        SszStaticHandler::<BeaconBlockBodyFulu<GnosisEthSpec>, GnosisEthSpec>::fulu_only().run();
    }

    // Altair and later
    #[test]
    fn contribution_and_proof() {
        SszStaticHandler::<ContributionAndProof<GnosisEthSpec>, GnosisEthSpec>::altair_and_later(
        )
        .run();
    }

    // LightClientBootstrap has no internal indicator of which fork it is for, so we test it separately.
    #[test]
    fn light_client_bootstrap() {
        SszStaticHandler::<LightClientBootstrapAltair<GnosisEthSpec>, GnosisEthSpec>::altair_only()
            .run();
        SszStaticHandler::<LightClientBootstrapAltair<GnosisEthSpec>, GnosisEthSpec>::bellatrix_only(
        )
        .run();
        SszStaticHandler::<LightClientBootstrapCapella<GnosisEthSpec>, GnosisEthSpec>::capella_only()
            .run();
        SszStaticHandler::<LightClientBootstrapDeneb<GnosisEthSpec>, GnosisEthSpec>::deneb_only()
            .run();
        SszStaticHandler::<LightClientBootstrapElectra<GnosisEthSpec>, GnosisEthSpec>::electra_only()
            .run();
        SszStaticHandler::<LightClientBootstrapFulu<GnosisEthSpec>, GnosisEthSpec>::fulu_only()
            .run();
    }

    // LightClientHeader has no internal indicator of which fork it is for, so we test it separately.
    #[test]
    fn light_client_header() {
        SszStaticHandler::<LightClientHeaderAltair<GnosisEthSpec>, GnosisEthSpec>::altair_only()
            .run();
        SszStaticHandler::<LightClientHeaderAltair<GnosisEthSpec>, GnosisEthSpec>::bellatrix_only()
            .run();

        SszStaticHandler::<LightClientHeaderCapella<GnosisEthSpec>, GnosisEthSpec>::capella_only(
        )
        .run();

        SszStaticHandler::<LightClientHeaderDeneb<GnosisEthSpec>, GnosisEthSpec>::deneb_only()
            .run();
        SszStaticHandler::<LightClientHeaderElectra<GnosisEthSpec>, GnosisEthSpec>::electra_only(
        )
        .run();
        SszStaticHandler::<LightClientHeaderFulu<GnosisEthSpec>, GnosisEthSpec>::fulu_only()
            .run();
    }

    // LightClientOptimisticUpdate has no internal indicator of which fork it is for, so we test it separately.
    #[test]
    fn light_client_optimistic_update() {
        SszStaticHandler::<LightClientOptimisticUpdateAltair<GnosisEthSpec>, GnosisEthSpec>::altair_only().run();
        SszStaticHandler::<LightClientOptimisticUpdateAltair<GnosisEthSpec>, GnosisEthSpec>::bellatrix_only().run();
        SszStaticHandler::<LightClientOptimisticUpdateCapella<GnosisEthSpec>, GnosisEthSpec>::capella_only().run();
        SszStaticHandler::<LightClientOptimisticUpdateDeneb<GnosisEthSpec>, GnosisEthSpec>::deneb_only().run();
        SszStaticHandler::<LightClientOptimisticUpdateElectra<GnosisEthSpec>, GnosisEthSpec>::electra_only().run();
        SszStaticHandler::<LightClientOptimisticUpdateFulu<GnosisEthSpec>, GnosisEthSpec>::fulu_only().run();
    }

    // LightClientFinalityUpdate has no internal indicator of which fork it is for, so we test it separately.
    #[test]
    fn light_client_finality_update() {
        SszStaticHandler::<LightClientFinalityUpdateAltair<GnosisEthSpec>, GnosisEthSpec>::altair_only(
        )
            .run();
        SszStaticHandler::<LightClientFinalityUpdateAltair<GnosisEthSpec>, GnosisEthSpec>::bellatrix_only(
        )
            .run();
        SszStaticHandler::<LightClientFinalityUpdateCapella<GnosisEthSpec>, GnosisEthSpec>::capella_only(
        )
            .run();
        SszStaticHandler::<LightClientFinalityUpdateDeneb<GnosisEthSpec>, GnosisEthSpec>::deneb_only(
        )
            .run();
        SszStaticHandler::<LightClientFinalityUpdateElectra<GnosisEthSpec>, GnosisEthSpec>::electra_only(
        )
            .run();
        SszStaticHandler::<LightClientFinalityUpdateFulu<GnosisEthSpec>, GnosisEthSpec>::fulu_only(
        )
            .run();
    }

    // LightClientUpdate has no internal indicator of which fork it is for, so we test it separately.
    #[test]
    fn light_client_update() {
        SszStaticHandler::<LightClientUpdateAltair<GnosisEthSpec>, GnosisEthSpec>::altair_only()
            .run();
        SszStaticHandler::<LightClientUpdateAltair<GnosisEthSpec>, GnosisEthSpec>::bellatrix_only()
            .run();
        SszStaticHandler::<LightClientUpdateCapella<GnosisEthSpec>, GnosisEthSpec>::capella_only(
        )
        .run();
        SszStaticHandler::<LightClientUpdateDeneb<GnosisEthSpec>, GnosisEthSpec>::deneb_only()
            .run();
        SszStaticHandler::<LightClientUpdateElectra<GnosisEthSpec>, GnosisEthSpec>::electra_only(
        )
        .run();
        SszStaticHandler::<LightClientUpdateFulu<GnosisEthSpec>, GnosisEthSpec>::fulu_only()
            .run();
    }

    #[test]
    fn signed_contribution_and_proof() {
        SszStaticHandler::<SignedContributionAndProof<GnosisEthSpec>, GnosisEthSpec>::altair_and_later().run();
    }

    #[test]
    fn sync_aggregate() {
        SszStaticHandler::<SyncAggregate<GnosisEthSpec>, GnosisEthSpec>::altair_and_later().run();
    }

    #[test]
    fn sync_committee() {
        SszStaticHandler::<SyncCommittee<GnosisEthSpec>, GnosisEthSpec>::altair_and_later().run();
    }

    #[test]
    fn sync_committee_contribution() {
        SszStaticHandler::<SyncCommitteeContribution<GnosisEthSpec>, GnosisEthSpec>::altair_and_later().run();
    }

    #[test]
    fn sync_committee_message() {
        SszStaticHandler::<SyncCommitteeMessage, GnosisEthSpec>::altair_and_later().run();
    }

    #[test]
    fn sync_aggregator_selection_data() {
        SszStaticHandler::<SyncAggregatorSelectionData, GnosisEthSpec>::altair_and_later().run();
    }

    // Bellatrix and later
    #[test]
    fn execution_payload() {
        SszStaticHandler::<ExecutionPayloadBellatrix<GnosisEthSpec>, GnosisEthSpec>::bellatrix_only()
            .run();
        SszStaticHandler::<ExecutionPayloadCapella<GnosisEthSpec>, GnosisEthSpec>::capella_only()
            .run();
        SszStaticHandler::<ExecutionPayloadDeneb<GnosisEthSpec>, GnosisEthSpec>::deneb_only()
            .run();
        SszStaticHandler::<ExecutionPayloadElectra<GnosisEthSpec>, GnosisEthSpec>::electra_only()
            .run();
        SszStaticHandler::<ExecutionPayloadFulu<GnosisEthSpec>, GnosisEthSpec>::fulu_only().run();
    }

    #[test]
    fn execution_payload_header() {
        SszStaticHandler::<ExecutionPayloadHeaderBellatrix<GnosisEthSpec>, GnosisEthSpec>::bellatrix_only()
            .run();
        SszStaticHandler::<ExecutionPayloadHeaderCapella<GnosisEthSpec>, GnosisEthSpec>
            ::capella_only().run();
        SszStaticHandler::<ExecutionPayloadHeaderDeneb<GnosisEthSpec>, GnosisEthSpec>
            ::deneb_only().run();
        SszStaticHandler::<ExecutionPayloadHeaderElectra<GnosisEthSpec>, GnosisEthSpec>
            ::electra_only().run();
        SszStaticHandler::<ExecutionPayloadHeaderFulu<GnosisEthSpec>, GnosisEthSpec>::fulu_only()
            .run();
    }

    #[test]
    fn withdrawal() {
        SszStaticHandler::<Withdrawal, GnosisEthSpec>::capella_and_later().run();
    }

    #[test]
    fn bls_to_execution_change() {
        SszStaticHandler::<BlsToExecutionChange, GnosisEthSpec>::capella_and_later().run();
    }

    #[test]
    fn signed_bls_to_execution_change() {
        SszStaticHandler::<SignedBlsToExecutionChange, GnosisEthSpec>::capella_and_later().run();
    }

    #[test]
    fn blob_sidecar() {
        SszStaticHandler::<BlobSidecar<GnosisEthSpec>, GnosisEthSpec>::deneb_and_later().run();
    }

    #[test]
    fn blob_identifier() {
        SszStaticHandler::<BlobIdentifier, GnosisEthSpec>::deneb_and_later().run();
    }

    #[test]
    fn historical_summary() {
        SszStaticHandler::<HistoricalSummary, GnosisEthSpec>::capella_and_later().run();
    }

    #[test]
    fn data_column_sidecar() {
        SszStaticHandler::<DataColumnSidecarFulu<GnosisEthSpec>, GnosisEthSpec>::fulu_only()
            .run();
        SszStaticHandler::<DataColumnSidecarGloas<GnosisEthSpec>, GnosisEthSpec>::gloas_only()
            .run();
    }

    #[test]
    fn data_column_by_root_identifier() {
        SszStaticWithSpecHandler::<
            DataColumnsByRootIdentifier<GnosisEthSpec>,
            GnosisEthSpec,
        >::fulu_and_later()
        .run();
    }

    #[test]
    fn consolidation() {
        SszStaticHandler::<ConsolidationRequest, GnosisEthSpec>::electra_and_later().run();
    }

    #[test]
    fn deposit_request() {
        SszStaticHandler::<DepositRequest, GnosisEthSpec>::electra_and_later().run();
    }

    #[test]
    fn withdrawal_request() {
        SszStaticHandler::<WithdrawalRequest, GnosisEthSpec>::electra_and_later().run();
    }

    #[test]
    fn pending_balance_deposit() {
        SszStaticHandler::<PendingDeposit, GnosisEthSpec>::electra_and_later().run();
    }

    #[test]
    fn pending_consolidation() {
        SszStaticHandler::<PendingConsolidation, GnosisEthSpec>::electra_and_later().run();
    }

    #[test]
    fn pending_partial_withdrawal() {
        SszStaticHandler::<PendingPartialWithdrawal, GnosisEthSpec>::electra_and_later().run();
    }

    #[test]
    fn execution_requests() {
        SszStaticHandler::<ExecutionRequests<GnosisEthSpec>, GnosisEthSpec>::electra_and_later()
            .run();
    }
}

#[test]
fn ssz_generic() {
    SszGenericHandler::<BasicVector>::default().run();
    SszGenericHandler::<Bitlist>::default().run();
    SszGenericHandler::<Bitvector>::default().run();
    SszGenericHandler::<Boolean>::default().run();
    SszGenericHandler::<Uints>::default().run();
    SszGenericHandler::<Containers>::default().run();
}

#[test]
fn epoch_processing_justification_and_finalization() {
    EpochProcessingHandler::<GnosisEthSpec, JustificationAndFinalization>::default().run();
}

#[test]
fn epoch_processing_rewards_and_penalties() {
    EpochProcessingHandler::<GnosisEthSpec, RewardsAndPenalties>::default().run();
}

#[test]
fn epoch_processing_registry_updates() {
    EpochProcessingHandler::<GnosisEthSpec, RegistryUpdates>::default().run();
}

#[test]
fn epoch_processing_slashings() {
    EpochProcessingHandler::<GnosisEthSpec, Slashings>::default().run();
}

#[test]
fn epoch_processing_eth1_data_reset() {
    EpochProcessingHandler::<GnosisEthSpec, Eth1DataReset>::default().run();
}

#[test]
fn epoch_processing_pending_balance_deposits() {
    EpochProcessingHandler::<GnosisEthSpec, PendingBalanceDeposits>::default().run();
}

#[test]
fn epoch_processing_pending_consolidations() {
    EpochProcessingHandler::<GnosisEthSpec, PendingConsolidations>::default().run();
}

#[test]
fn epoch_processing_effective_balance_updates() {
    EpochProcessingHandler::<GnosisEthSpec, EffectiveBalanceUpdates>::default().run();
}

#[test]
fn epoch_processing_slashings_reset() {
    EpochProcessingHandler::<GnosisEthSpec, SlashingsReset>::default().run();
}

#[test]
fn epoch_processing_randao_mixes_reset() {
    EpochProcessingHandler::<GnosisEthSpec, RandaoMixesReset>::default().run();
}

#[test]
fn epoch_processing_historical_roots_update() {
    EpochProcessingHandler::<GnosisEthSpec, HistoricalRootsUpdate>::default().run();
}

#[test]
fn epoch_processing_historical_summaries_update() {
    EpochProcessingHandler::<GnosisEthSpec, HistoricalSummariesUpdate>::default().run();
}

#[test]
fn epoch_processing_participation_record_updates() {
    EpochProcessingHandler::<GnosisEthSpec, ParticipationRecordUpdates>::default().run();
}

#[test]
fn epoch_processing_sync_committee_updates() {
    // There are presently no mainnet tests, see:
    // https://github.com/ethereum/consensus-spec-tests/issues/29
    EpochProcessingHandler::<GnosisEthSpec, SyncCommitteeUpdates>::default().run();
}

#[test]
fn epoch_processing_inactivity_updates() {
    EpochProcessingHandler::<GnosisEthSpec, InactivityUpdates>::default().run();
}

#[test]
fn epoch_processing_participation_flag_updates() {
    EpochProcessingHandler::<GnosisEthSpec, ParticipationFlagUpdates>::default().run();
}

#[test]
fn epoch_processing_proposer_lookahead() {
    EpochProcessingHandler::<GnosisEthSpec, ProposerLookahead>::default().run();
}

#[test]
fn fork_upgrade() {
    ForkHandler::<GnosisEthSpec>::default().run();
}

#[test]
fn transition() {
    TransitionHandler::<GnosisEthSpec>::default().run();
}

#[test]
fn finality() {
    FinalityHandler::<GnosisEthSpec>::default().run();
}

#[test]
fn fork_choice_get_head() {
    ForkChoiceHandler::<GnosisEthSpec>::new("get_head").run();
}

#[test]
fn fork_choice_on_block() {
    ForkChoiceHandler::<GnosisEthSpec>::new("on_block").run();
}

#[test]
fn fork_choice_ex_ante() {
    ForkChoiceHandler::<GnosisEthSpec>::new("ex_ante").run();
}

#[test]
fn fork_choice_reorg() {
    ForkChoiceHandler::<GnosisEthSpec>::new("reorg").run();
    // There is no mainnet variant for this test.
}

#[test]
fn fork_choice_withholding() {
    ForkChoiceHandler::<GnosisEthSpec>::new("withholding").run();
    // There is no mainnet variant for this test.
}

#[test]
fn fork_choice_should_override_forkchoice_update() {
    ForkChoiceHandler::<GnosisEthSpec>::new("should_override_forkchoice_update").run();
}

#[test]
fn fork_choice_get_proposer_head() {
    ForkChoiceHandler::<GnosisEthSpec>::new("get_proposer_head").run();
}

#[test]
fn fork_choice_deposit_with_reorg() {
    ForkChoiceHandler::<GnosisEthSpec>::new("deposit_with_reorg").run();
    // There is no mainnet variant for this test.
}

#[test]
fn optimistic_sync() {
    OptimisticSyncHandler::<GnosisEthSpec>::default().run();
}

#[test]
fn genesis_initialization() {
    GenesisInitializationHandler::<GnosisEthSpec>::default().run();
}

#[test]
fn genesis_validity() {
    GenesisValidityHandler::<GnosisEthSpec>::default().run();
    // Note: there are no genesis validity tests for mainnet
}

#[test]
fn kzg_blob_to_kzg_commitment() {
    KZGBlobToKZGCommitmentHandler::<GnosisEthSpec>::default().run();
}

#[test]
fn kzg_compute_blob_kzg_proof() {
    KZGComputeBlobKZGProofHandler::<GnosisEthSpec>::default().run();
}

#[test]
fn kzg_compute_kzg_proof() {
    KZGComputeKZGProofHandler::<GnosisEthSpec>::default().run();
}

#[test]
fn kzg_verify_blob_kzg_proof() {
    KZGVerifyBlobKZGProofHandler::<GnosisEthSpec>::default().run();
}

#[test]
fn kzg_verify_blob_kzg_proof_batch() {
    KZGVerifyBlobKZGProofBatchHandler::<GnosisEthSpec>::default().run();
}

#[test]
fn kzg_verify_kzg_proof() {
    KZGVerifyKZGProofHandler::<GnosisEthSpec>::default().run();
}

#[test]
fn kzg_compute_cells() {
    KZGComputeCellsHandler::<GnosisEthSpec>::default().run();
}

#[test]
fn kzg_compute_cells_and_proofs() {
    KZGComputeCellsAndKZGProofHandler::<GnosisEthSpec>::default().run();
}

#[test]
fn kzg_verify_cell_proof_batch() {
    KZGVerifyCellKZGProofBatchHandler::<GnosisEthSpec>::default().run();
}

#[test]
fn kzg_recover_cells_and_proofs() {
    KZGRecoverCellsAndKZGProofHandler::<GnosisEthSpec>::default().run();
}

#[test]
fn light_client_merkle_proof_validity() {
    MerkleProofValidityHandler::<GnosisEthSpec>::default().run();
}

#[test]
fn light_client_update() {
    LightClientUpdateHandler::<GnosisEthSpec>::default().run();
}

#[test]
#[cfg(feature = "fake_crypto")]
fn kzg_inclusion_merkle_proof_validity() {
    KzgInclusionMerkleProofValidityHandler::<GnosisEthSpec>::default().run();
}

#[test]
fn rewards() {
    for handler in &["basic", "leak", "random"] {
        RewardsHandler::<GnosisEthSpec>::new(handler).run();
    }
}

#[test]
fn get_custody_groups() {
    GetCustodyGroupsHandler::<GnosisEthSpec>::default().run();
    GetCustodyGroupsHandler::<GnosisEthSpec>::default().run()
}

#[test]
fn compute_columns_for_custody_group() {
    ComputeColumnsForCustodyGroupHandler::<GnosisEthSpec>::default().run();
    ComputeColumnsForCustodyGroupHandler::<GnosisEthSpec>::default().run();
}

