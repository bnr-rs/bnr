use crate::create_xfs_struct;
use crate::{
    BadShapeRejectCount, BillExtractedCount, ExtractionRejectCount, RecognitionRejectCount, Slots,
    TransportRejectCount, TransportRejectDetails,
};

create_xfs_struct!(
    LoaderAcceptanceHistory,
    "loaderAcceptanceHistory",
    [
        bill_extracted_count: BillExtractedCount,
        extraction_reject_count: ExtractionRejectCount,
        transport_reject_count: TransportRejectCount,
        transport_reject_details: TransportRejectDetails,
        recognition_reject_count: RecognitionRejectCount,
        bad_shape_reject_count: BadShapeRejectCount,
        slots: Slots
    ],
    "Acceptance bills extracted from the loader unit."
);
