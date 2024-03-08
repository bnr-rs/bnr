use crate::create_xfs_struct;
use crate::{
    BadRoughShapeCount, BadShapeCount, ForcedInCount, HeldBackCount, InletDetectionCount,
    RemovedCount, StringDetectionCount, TooThickCount,
};

create_xfs_struct!(
    InsertionRejectDetails,
    "insertionRejectDetails",
    [
        forced_in_count: ForcedInCount,
        removed_count: RemovedCount,
        held_back_count: HeldBackCount,
        too_thick_count: TooThickCount,
        bad_rough_shape_count: BadRoughShapeCount,
        bad_shape_count: BadShapeCount,
        string_detection_count: StringDetectionCount,
        inlet_detection_count: InletDetectionCount
    ],
    "Represents details about inssertion reject events."
);
