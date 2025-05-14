use crate::per_epoch_processing::{Error, JustificationAndFinalizationState};
use safe_arith::SafeArith;
use std::ops::Range;
use types::{Checkpoint, EthSpec};

/// Update the justified and finalized checkpoints for matching target attestations.
#[allow(clippy::if_same_then_else)] // For readability and consistency with spec.
pub fn weigh_justification_and_finalization<E: EthSpec>(
    mut state: JustificationAndFinalizationState<E>,
    total_active_balance: u64,
    previous_target_balance: u64,
    current_target_balance: u64,
) -> Result<JustificationAndFinalizationState<E>, Error> {
    let previous_epoch = state.previous_epoch();
    let current_epoch = state.current_epoch();

    let old_previous_justified_checkpoint = state.previous_justified_checkpoint();
    let old_current_justified_checkpoint = state.current_justified_checkpoint();

    // Process justifications
    *state.previous_justified_checkpoint_mut() = state.current_justified_checkpoint();
    state.justification_bits_mut().shift_up(1)?;

    if previous_target_balance.safe_mul(3)? >= total_active_balance.safe_mul(2)? {
        *state.current_justified_checkpoint_mut() = Checkpoint {
            epoch: previous_epoch,
            root: state.get_block_root_at_epoch(previous_epoch)?,
        };
        state.justification_bits_mut().set(1, true)?;
    }
    // If the current epoch gets justified, fill the last bit.
    if current_target_balance.safe_mul(3)? >= total_active_balance.safe_mul(2)? {
        *state.current_justified_checkpoint_mut() = Checkpoint {
            epoch: current_epoch,
            root: state.get_block_root_at_epoch(current_epoch)?,
        };
        state.justification_bits_mut().set(0, true)?;
    }

    let bits = state.justification_bits().clone();
    let all_bits_set = |range: Range<usize>| -> Result<bool, Error> {
        for i in range {
            if !bits.get(i).map_err(Error::InvalidJustificationBit)? {
                return Ok(false);
            }
        }
        Ok(true)
    };

    // The 2nd/3rd/4th most recent epochs are all justified, the 2nd using the 4th as source.
    if all_bits_set(1..4)? && old_previous_justified_checkpoint.epoch.safe_add(3)? == current_epoch
    {
        *state.finalized_checkpoint_mut() = old_previous_justified_checkpoint;
    }
    // The 2nd/3rd most recent epochs are both justified, the 2nd using the 3rd as source.
    if all_bits_set(1..3)? && old_previous_justified_checkpoint.epoch.safe_add(2)? == current_epoch
    {
        *state.finalized_checkpoint_mut() = old_previous_justified_checkpoint;
    }
    // The 1st/2nd/3rd most recent epochs are all justified, the 1st using the 3nd as source.
    if all_bits_set(0..3)? && old_current_justified_checkpoint.epoch.safe_add(2)? == current_epoch {
        *state.finalized_checkpoint_mut() = old_current_justified_checkpoint;
    }
    // The 1st/2nd most recent epochs are both justified, the 1st using the 2nd as source.
    if all_bits_set(0..2)? && old_current_justified_checkpoint.epoch.safe_add(1)? == current_epoch {
        *state.finalized_checkpoint_mut() = old_current_justified_checkpoint;
    }

    Ok(state)
}

/// Update the justified and finalized checkpoints for matching target attestations.
#[allow(clippy::if_same_then_else)] // For readability and consistency with spec.
pub fn weigh_justification_and_finalization_fulu<E: EthSpec>(
    mut state: JustificationAndFinalizationState<E>,
    total_active_balance: u64,
    previous_target_balance: u64,
    current_target_balance: u64,
) -> Result<JustificationAndFinalizationState<E>, Error> {
    // Process justifications
    // Register that this epoch received enough votes to justify. But don't update the justified
    // checkpoint until the upstream chain justified it.
    if previous_epoch_target_balance * 3 >= total_active_balance * 2 {
        state.justified_checkpoints.push(Checkpoint {
            epoch: previous_epoch,
            root: get_block_root(state, previous_epoch),
        });
    }

    if current_epoch_target_balance * 3 >= total_active_balance * 2 {
        state.justified_checkpoints.push(Checkpoint {
            epoch: current_epoch,
            root: get_block_root(state, current_epoch),
        });
    }

    // We can only justify a checkpoint if its block point to an upstream block that is upstream justified.
    // We know that all ancestor blocks of this chain point to upstream canonical blocks.
    // We can justify only epochs in `voted_justified_epochs` set whose block points to an upstream block with
    // a slot <= than the slot of the upstream justified epoch.
    for checkpoint in state.justified_checkpoints().iter().reverse() {
        // `epoch` is in the L2 chain's time
        // The justified block of `epoch` has a timestamp <= than `timestamp(start_slot(epoch))`
        // `state.upstream_justified_block_timestamp` is in the upstream chain's time
        // Any L2 block must point to an upstream block that has <= timestamp
        // TODO: optimization: iterate in reverse and stop at the first match
        let checkpoint_block_timestamp =
            compute_timestamp_at_slot(compute_start_slot_at_epoch(checkpoint.epoch));
        let upstream_justified_block_timestamp = compute_upstream_timestamp_at_slot(
            compute_upstream_start_slot_at_epoch(state.upstream_justified_checkpoint.epoch),
        );
        if checkpoint_block_timestamp <= upstream_justified_block_timestamp {
            // The block `checkpoint.root` must point to an upstream block that is justified
            *state.current_justified_checkpoint_mut() = checkpoint;
            break;
        }
    }

    // Process finalizations
    if let Some(finalized_checkpoint) = satisfy_finality(
        state.upstream_finalized_checkpoint,
        state.justified_checkpoints,
    ) {
        *state.finalized_checkpoint_mut() = finalized_checkpoint;
        // Prune `justified_checkpoints` that are too old
        *state.justified_checkpoints_mut() = state
            .justified_checkpoints()
            .iter()
            .filter(|cp| cp.epoch > finalized_checkpoint.epoch)
            .collect::<Vec<_>>();
    }

    Ok(())
}

fn satisfy_finality(
    upstream_finalized_checkpoint: Checkpoint,
    justified_checkpoint: &[Checkpoint],
) -> Option<Checkpoint> {
    todo!();
}
