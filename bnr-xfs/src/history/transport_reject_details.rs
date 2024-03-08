use crate::create_xfs_struct;
use crate::{PositioningFailedCount, SystemEventCount, TransportEventCount};

create_xfs_struct!(
    TransportRejectDetails,
    "transportRejectDetails",
    [
        positioning_failed_count: PositioningFailedCount,
        transport_event_count: TransportEventCount,
        system_event_count: SystemEventCount
    ],
    "Represents the details of transport reject events."
);
