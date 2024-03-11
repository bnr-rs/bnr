//! Contains count types for the XFS protocol.

use crate::create_xfs_i4;

create_xfs_i4!(Count, "count", "Represents the note count.");

create_xfs_i4!(
    InitialCount,
    "initialCount",
    "Represents the initial note count."
);

create_xfs_i4!(
    TransportCount,
    "transportCount",
    "Represents the transport count."
);

create_xfs_i4!(
    DepositCount,
    "depositCount",
    "Represents the deposit count."
);

create_xfs_i4!(
    RetractedCount,
    "retractedCount",
    "Represents the retracted count."
);

create_xfs_i4!(
    EmptiedCount,
    "emptiedCount",
    "Represents the emptied count."
);

create_xfs_i4!(
    ForgeryCount,
    "forgeryCount",
    "Represents the forgery count."
);

create_xfs_i4!(
    DisappearedCount,
    "disappearedCount",
    "Represents the disappeared count."
);

create_xfs_i4!(
    DispenseCount,
    "dispenseCount",
    "Represents the dispense count."
);

create_xfs_i4!(RejectCount, "rejectCount", "Represents the reject count.");

create_xfs_i4!(
    InsertionStartCount,
    "insertionStartCount",
    "Represents the start count for bill insertion."
);

create_xfs_i4!(
    InsertionRejectCount,
    "insertionRejectCount",
    "Represents the reject count for bill insertion."
);

create_xfs_i4!(
    CancelRejectCount,
    "cancelRejectCount",
    "Represents the cancel reject count."
);

create_xfs_i4!(
    UnknownCount,
    "unknownCount",
    "Represents the unknown count."
);

create_xfs_i4!(
    UnknownRejectCount,
    "unknownRejectCount",
    "Represents the unknown reject count."
);

create_xfs_i4!(
    ConfusionCount,
    "confusionCount",
    "Represents the confusion count."
);

create_xfs_i4!(
    FitnessCount,
    "fitnessCount",
    "Represents the fitness count."
);

create_xfs_i4!(
    FitnessRejectCount,
    "fitnessRejectCount",
    "Represents the fitness reject count."
);

create_xfs_i4!(ValidCount, "validCount", "Represents the valid count.");

create_xfs_i4!(
    ValidUnfitCount,
    "validUnfitCount",
    "Represents the valid unfit count."
);

create_xfs_i4!(
    SuspectCount,
    "suspectCount",
    "Represents the suspect count."
);

create_xfs_i4!(
    StainedCount,
    "stainedCount",
    "Represents the stained count."
);

create_xfs_i4!(
    ConfigurationRejectCount,
    "configurationRejectCount",
    "Represents the configuration reject count."
);

create_xfs_i4!(
    BillExtractedCount,
    "billExtractedCount",
    "Represents the bill extracted count."
);

create_xfs_i4!(
    BillRolledBackCount,
    "billRolledBackCount",
    "Represents the bill rolled back count."
);

create_xfs_i4!(
    CashInTransactionCount,
    "cashInTransactionCount",
    "Represents the cash in transaction count."
);

create_xfs_i4!(
    TransportRejectCount,
    "transportRejectCount",
    "Represents the transport reject count."
);

create_xfs_i4!(
    TransportEventCount,
    "transportEventCount",
    "Represents the transport event count."
);

create_xfs_i4!(
    ExtractionRejectCount,
    "extractionRejectCount",
    "Represents the extraction reject count."
);

create_xfs_i4!(
    RecognitionRejectCount,
    "recognitionRejectCount",
    "Represents the recognition reject count."
);

create_xfs_i4!(
    PositioningFailedCount,
    "positioningFailedCount",
    "Represents the positioning failed count."
);

create_xfs_i4!(
    SystemEventCount,
    "systemEventCount",
    "Represents the system event count."
);

create_xfs_i4!(
    ForcedInCount,
    "forcedInCount",
    "Represents the forced in count."
);

create_xfs_i4!(
    RemovedCount,
    "removedCount",
    "Represents the removed count."
);

create_xfs_i4!(
    HeldBackCount,
    "heldBackCount",
    "Represents the held back count."
);

create_xfs_i4!(
    TooThickCount,
    "tooThickCount",
    "Represents the too thick count."
);

create_xfs_i4!(
    TooLongCount,
    "tooLongCount",
    "Represents the too long count."
);

create_xfs_i4!(
    BadRoughShapeCount,
    "badRoughShapeCount",
    "Represents the bad rough shape count."
);

create_xfs_i4!(
    BadShapeCount,
    "badShapeCount",
    "Represents the bad shape count."
);

create_xfs_i4!(
    BadShapeRejectCount,
    "badShapeRejectCount",
    "Represents the bad shape reject count."
);

create_xfs_i4!(
    StringDetectionCount,
    "stringDetectionCount",
    "Represents the string detection count."
);

create_xfs_i4!(
    InletDetectionCount,
    "inletDetectionCount",
    "Represents the inlet detection count."
);

create_xfs_i4!(
    SuperimposedCount,
    "superimposedCount",
    "Represents the superimposed count."
);

create_xfs_i4!(
    OtherDenominationCount,
    "otherDenominationCount",
    "Represents the other denomination count."
);

create_xfs_i4!(
    StackedWhileRecyclerFullCount,
    "stackedWhileRecyclerFullCount",
    "Represents the stacked while recycler full count."
);

create_xfs_i4!(
    DenominateAmountCount,
    "denominateAmountCount",
    "Represents the denominate amount count."
);

create_xfs_i4!(
    AmountNotAvailableCount,
    "amountNotAvailableCount",
    "Represents the amount not available count."
);

create_xfs_i4!(
    BillRequestedCount,
    "billRequestedCount",
    "Represents the bill requested count."
);

create_xfs_i4!(
    BillNotAvailableCount,
    "billNotAvailableCount",
    "Represents the bill not available count."
);

create_xfs_i4!(
    TooManyBillsCount,
    "tooManyBillsCount",
    "Represents the too many bills count."
);

create_xfs_i4!(
    DirectFromLoaderCount,
    "directFromLoaderCount",
    "Represents the direct from loader count."
);

create_xfs_i4!(
    DispenseAmountCount,
    "dispenseAmountCount",
    "Represents the dispense amount count."
);

create_xfs_i4!(
    HardwareFailureCount,
    "hardwareFailureCount",
    "Represents the hardware failure count."
);

create_xfs_i4!(
    HardwareFailureWithBillStoppedCount,
    "hardwareFailureWithBillStoppedCount",
    "Represents the hardware failure with bill stopped count."
);

create_xfs_i4!(
    OperationalDegradedCount,
    "operationalDegradedCount",
    "Represents the operational degraded count."
);

create_xfs_i4!(
    BillJamCount,
    "billJamCount",
    "Represents the bill jam count."
);

create_xfs_i4!(
    EnvironmentErrorCount,
    "environmentErrorCount",
    "Represents the environment error count."
);

create_xfs_i4!(
    BillErrorCount,
    "billErrorCount",
    "Represents the bill error count."
);

create_xfs_i4!(
    TransportErrorCount,
    "transportErrorCount",
    "Represents the transport error count."
);

create_xfs_i4!(
    BillTooShortInBottomTransportBwCount,
    "billTooShortInBottomTransportBwCount",
    "Represents the bill too short in bottom transport bw count."
);

create_xfs_i4!(
    BillTooLongInBottomTransportBwCount,
    "billTooLongInBottomTransportBwCount",
    "Represents the bill too long in bottom transport bw count."
);

create_xfs_i4!(
    BillTooShortInSpineFwCount,
    "billTooShortInSpineFwCount",
    "Represents the bill too short in spine fw count."
);

create_xfs_i4!(
    BillTooLongInSpineFwCount,
    "billTooLongInSpineFwCount",
    "Represents the bill too long in spine fw count."
);

create_xfs_i4!(
    MissingModuleCount,
    "missingModuleCount",
    "Represents the missile module count."
);

create_xfs_i4!(
    ConfigurationErrorCount,
    "configurationErrorCount",
    "Represents the configuration error count."
);

create_xfs_i4!(
    IncompatibleSoftwareCount,
    "incompatibleSoftwareCount",
    "Represents the incompatible software count."
);

create_xfs_i4!(
    ResetWithCoverOpenCount,
    "resetWithCoverOpenCount",
    "Represents the reset with cover open count."
);

create_xfs_i4!(
    ResetWithInterlockOpenCount,
    "resetWithInterlockOpenCount",
    "Represents the reset with interlock open count."
);

create_xfs_i4!(
    PositionerCount,
    "positionerCount",
    "Represents the positioner count."
);

create_xfs_i4!(
    RecognitionSystemCount,
    "recognitionSystemCount",
    "Represents the recognition system count."
);

create_xfs_i4!(
    BottomTransportCount,
    "bottomTransportCount",
    "Represents the bottom transport count."
);

create_xfs_i4!(
    BundlerCount,
    "bundlerCount",
    "Represents the bundler count."
);

create_xfs_i4!(ModuleCount, "moduleCount", "Represents the module count.");

create_xfs_i4!(
    InterfaceCount,
    "interfaceCount",
    "Represents the interface count."
);

create_xfs_i4!(SpineCount, "spineCount", "Represents the spine count.");

create_xfs_i4!(UnnamedCount, "", "Represents an unnamed count.");
