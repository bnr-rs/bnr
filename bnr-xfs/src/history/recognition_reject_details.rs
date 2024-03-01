use crate::create_xfs_struct;
use crate::ForgeryCount;

use super::UnknownCount;

create_xfs_struct!(
    RecognitionRejectDetails,
    "recognitionRejectDetails",
    [
        forgery_count: ForgeryCount,
        unknown_count: UnknownCount
    ],
    "Represents the details of recognition reject events."
);
