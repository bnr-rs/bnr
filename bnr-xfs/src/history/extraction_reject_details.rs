use crate::create_xfs_struct;
use crate::{SuperimposedCount, TooLongCount};

create_xfs_struct!(
    ExtractionRejectDetails,
    "extractionRejectDetails",
    [
        too_long_count: TooLongCount,
        superimposed_count: SuperimposedCount
    ],
    "Represents the details of extraction reject events."
);
