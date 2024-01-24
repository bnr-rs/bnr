/// Represents BNR error codes.
///
/// Documentation for each variant includes recommendations for resolving the error.
#[allow(non_camel_case_types)]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BnrError {
    /// E_SS01: BNR is idle mode
    ///
    /// Corrective action(s):
    ///
    /// - Send Reset command
    E_SS01  = 536870993,
    /// E_SS02: BNR is starting up
    ///
    /// Corrective action(s):
    ///
    /// - Wait for reset to complete
    E_SS02 = 536870995,
    /// E_SS03: BNR Interlock is open
    ///
    /// Corrective action(s):
    ///
    /// - Close the Main BNR  Lock
    E_SS03 = 536870994,
    /// E_MM01: A software error has ocurred
    ///
    /// Corrective action(s):
    ///
    /// - Send BNR Configuration file to unit
    E_MM01 = 234881105,
    /// E_MM03: A software error has ocurred
    ///
    /// Corrective action(s):
    ///
    /// - Send BNR Configuration file to unit
    E_MM03 = 234881107,
    /// E_MM04: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Remove banknote from intlet.
    E_MM04 = 50462737,
    /// E_MM05: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Open Top Cover or Recognition Module. Remove banknote.
    E_MM05 = 50724881,
    /// E_MM06: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Open Top Cover or Recognition Module. Remove banknote.
    E_MM06 = 50462993,
    /// E_MM07: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Open Top Cover or Recognition Module. Remove banknote.
    E_MM07 = 50398481,
    /// E_MM08: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Open Top Cover or Recognition Module. Remove banknote.
    E_MM08 = 50397201,
    /// E_MM09: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Remove cashbox and Main Module. Remove banknote  from bottom of main module
    E_MM09 = 50398993,
    /// E_MM10: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Remove banknote from Outlet. (was Open Top Cover or Recognition Module. Remove banknote.)
    E_MM10 = 50399249,
    /// E_MM11: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Remove cashbox and Main Module. Remove banknote  from bottom of main module
    E_MM11 = 50400017,
    /// E_MM12: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Open Top Cover or Recognition Module. Remove banknote.
    E_MM12 = 50462738,
    /// E_MM13: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Open Top Cover or Recognition Module. Remove banknote.
    E_MM13 = 50724882,
    /// E_MM14: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Open Top Cover or Recognition Module. Remove banknote.
    E_MM14 = 50462994,
    /// E_MM15: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Open Top Cover or Recognition Module. Remove banknote.
    E_MM15 = 50398482,
    /// E_MM16: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Open Top Cover or Recognition Module. Remove banknote.
    E_MM16 = 50397202,
    /// E_MM17: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Remove cashbox and Main Module. Remove banknote  from bottom of main module
    E_MM17 = 50398994,
    /// E_MM18: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Remove banknote from outlet.
    E_MM18 = 50399250,
    /// E_MM19: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Remove cashbox and Main Module. Remove banknote  from bottom of main module
    E_MM19 = 50400018,
    /// E_MM20: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Open Top Cover or Recognition Module. Remove banknote.
    E_MM20 = 50462739,
    /// E_MM21: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Remove banknote from intlet.
    E_MM21 = 50724883,
    /// E_MM22: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Open Top Cover or Recognition Module. Remove banknote.
    E_MM22 = 50462995,
    /// E_MM23: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Open Top Cover or Recognition Module. Remove banknote.
    E_MM23 = 50398483,
    /// E_MM24: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Open Top Cover or Recognition Module. Remove banknote.
    E_MM24 = 50397203,
    /// E_MM25: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Remove cashbox and Main Module. Remove banknote  from bottom of main module
    E_MM25 = 50398995,
    /// E_MM26: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Remove banknote from outlet.
    E_MM26 = 50399251,
    /// E_MM27: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Remove cashbox and Main Module. Remove banknote  from bottom of main module
    E_MM27 = 50400019,
    /// E_MM28: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Open Top Cover or Recognition Module. Remove banknote.
    E_MM28 = 50462740,
    /// E_MM29: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Remove banknote from inlet.
    E_MM29 = 50724884,
    /// E_MM30: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Open Top Cover or Recognition Module. Remove banknote.
    E_MM30 = 50462996,
    /// E_MM31: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Open Top Cover or Recognition Module. Remove banknote.
    E_MM31 = 50398484,
    /// E_MM32: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Open Top Cover or Recognition Module. Remove banknote.
    E_MM32 = 50397204,
    /// E_MM33: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Remove cashbox and Main Module. Remove banknote  from bottom of main module
    E_MM33 = 50398996,
    /// E_MM34: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Remove banknote from outlet.
    E_MM34 = 50399252,
    /// E_MM35: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Remove cashbox and Main Module. Remove banknote  from bottom of main module
    E_MM35 = 50400020,
    /// E_MM41: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove cashbox. Remove object (if any) blocking the Stacking Mechanism.
    ///   - If problem persists: Main Module needs to be replaced
    E_MM41 = 201326593,
    /// E_MM42: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove cashbox. Remove object (if any) blocking the Stacking Mechanism.
    ///   - If problem persists: Main Module needs to be replaced
    E_MM42 = 201326595,
    /// E_MM43: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove cashbox. Remove object (if any) blocking the Stacking Mechanism.
    ///   - If problem persists: Main Module needs to be replaced
    E_MM43 = 201326596,
    /// E_MM44: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove cashbox. Remove object (if any) blocking the Stacking Mechanism.
    ///   - If problem persists: Main Module needs to be replaced
    E_MM44 = 201326597,
    /// E_MM45: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove cashbox. Remove object (if any) blocking the Stacking Mechanism.
    ///   - If problem persists: Main Module needs to be replaced
    E_MM45 = 201326598,
    /// E_MM46: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove cashbox. Remove object (if any) blocking the Stacking Mechanism.
    ///   - If problem persists: Main Module needs to be replaced
    E_MM46 = 201326599,
    /// E_MM47: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove main module and cashbox using "stacker blocked in cashbox - removal procedure"
    ///   - If problem persists: Main Module needs to be replaced
    E_MM47 = 201326600,
    /// E_MM48: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove cashbox. Remove object (if any) blocking the Stacking Mechanism.
    ///   - If problem persists: Main Module needs to be replaced
    E_MM48 = 201326601,
    /// E_MM49: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove cashbox. Remove object (if any) blocking the Stacking Mechanism.
    ///   - If problem persists: Main Module needs to be replaced
    E_MM49 = 201326602,
    /// E_MM50: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Replace Cashbox with emtpy Cashbox
    ///
    /// - Remove cashbox. Remove object (if any) blocking the Stacking Mechanism.
    ///   - If problem persists: Main Module needs to be replaced
    E_MM50 = 201326603,
    /// E_MM51: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Encash the note with Reject.
    E_MM51 = 201326604,
    /// E_MM52: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove cashbox. Remove unexpected object (if any) in the Bundler Area. Secure BNR.
    ///   - If problem persists: Main Module needs to be replaced
    E_MM52 = 33554433,
    /// E_MM53: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove cashbox. Remove unexpected object (if any) in the Bundler Area. Secure BNR.
    ///   - If problem persists: Main Module needs to be replaced
    E_MM53 = 33554434,
    /// E_MM54: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove cashbox. Remove unexpected object (if any) in the Bundler Area. Secure BNR.
    ///   - If problem persists: Main Module needs to be replaced
    E_MM54 = 33554435,
    /// E_MM55: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove cashbox. Remove unexpected object (if any) in the Bundler Area. Secure BNR.
    ///   - If problem persists: Main Module needs to be replaced
    E_MM55 = 33554436,
    /// E_MM56: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove cashbox. Remove unexpected object (if any) in the Bundler Area. Secure BNR.
    ///   - If problem persists: Main Module needs to be replaced
    E_MM56 = 33554437,
    /// E_MM57: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove cashbox. Remove unexpected object (if any) in the Bundler Area. Secure BNR.
    ///   - If problem persists: Main Module needs to be replaced
    E_MM57 = 33554438,
    /// E_MM58: The Main Module cannot recover from an error
    ///
    /// Corrective action(s):
    ///
    /// - Open Recognition Module. Check if flat cable is correctly connected. If Power is on, Power off, insert the flat cable correctly
    ///   - If problem persists: Main Module needs to be replaced
    E_MM58 = 100663297,
    /// E_MM59: The Main Module cannot recover from an error
    ///
    /// Corrective action(s):
    ///
    /// - Main Module needs to be replaced
    E_MM59 = 100663298,
    /// E_MM60: The Main Module cannot recover from an error
    ///
    /// Corrective action(s):
    ///
    /// - Switch off very luminous light close to Recognition Module (if any).
    ///   - If problem persists: Main Module needs to be replaced
    E_MM60 = 100663299,
    /// E_MM61: The Main Module cannot recover from an error
    ///
    /// Corrective action(s):
    ///
    /// - Open Recognition sensor module. Remove unexpected object (if any), clean the sensor.
    ///   - If problem persists: Main Module needs to be replaced
    E_MM61 = 100663300,
    /// E_MM62: The Main Module cannot recover from an error
    ///
    /// Corrective action(s):
    ///
    /// - Main Module needs to be replaced
    E_MM62 = 100663301,
    /// E_MM63: The Main Module cannot recover from an error
    ///
    /// Corrective action(s):
    ///
    /// - Main Module needs to be replaced
    E_MM63 = 100663302,
    /// E_MM64: The Main Module cannot recover from an error
    ///
    /// Corrective action(s):
    ///
    /// - Main Module needs to be replaced
    E_MM64 = 100663303,
    /// E_MM65: The Main Module cannot recover from an error
    ///
    /// Corrective action(s):
    ///
    /// - Main Module needs to be replaced
    E_MM65 = 100663304,
    /// E_MM66: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Open Recognition sensor module. Remove unexpected object (if any), clean the sensor. 
    ///   - If problem persists: Main Module needs to be replaced
    E_MM66 = 100663305,
    /// E_MM39: BNR has detected an open cover which is preventing operation
    ///
    /// Corrective action(s):
    ///
    /// - Close open cover
    ///
    /// - Main Module needs to be replaced
    E_MM39 = 251658561,
    /// E_MM40: BNR has detected an open cover which is preventing operation
    ///
    /// Corrective action(s):
    ///
    /// - Close open cover
    ///
    /// - Main Module needs to be replaced
    E_MM40 = 251658305,
    /// E_MM67: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Open all covers, remove object (if any) from the bill path, close covers.
    ///   - If problem persists: Main Module needs to be replaced
    E_MM67 = 16908546,
    /// E_MM68: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Open all covers, remove object (if any) from the bill path, close covers.
    ///   - If problem persists: Main Module needs to be replaced
    E_MM68 = 16908547,
    /// E_MM69: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Open all covers, remove object (if any) from the bill path
    ///   - If problem persists: Main Module needs to be replaced
    E_MM69 = 16842754,
    /// E_MM70: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Open all covers, remove object (if any) from the bill path
    ///   - If problem persists: Main Module needs to be replaced
    E_MM70 = 16842755,
    /// E_MM71: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Open Top Cover, remove object (if any) from the inlet area, clean inlet
    ///
    /// - Main Module needs to be replaced
    E_MM71 = 50462721,
    /// E_MM72: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Open Top Cover, remove object (if any) from the inlet area, clean inlet
    ///
    /// - Main Module needs to be replaced
    E_MM72 = 50462722,
    /// E_MM73: The Main Module cannot recover from an error
    ///
    /// Corrective action(s):
    ///
    /// - Main Module needs to be replaced
    E_MM73 = 50724865,
    /// E_MM74: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Open Top Cover, remove object (if any) from the inlet area, clean inlet
    ///
    /// - Main Module needs to be replaced
    E_MM74 = 50724866,
    /// E_MM75: The Main Module cannot recover from an error
    ///
    /// Corrective action(s):
    ///
    /// - Main Module needs to be replaced
    E_MM75 = 50462977,
    /// E_MM76: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Open Top Cover, remove object (if any) from the inlet area, clean inlet
    ///
    /// - Main Module needs to be replaced
    E_MM76 = 50462978,
    /// E_MM77: The Main Module cannot recover from an error
    ///
    /// Corrective action(s):
    ///
    /// - Main Module needs to be replaced
    E_MM77 = 50398465,
    /// E_MM78: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Open Top Cover, remove object (if any) from the inlet area, clean inlet
    ///
    /// - Main Module needs to be replaced
    E_MM78 = 50398466,
    /// E_MM79: The Main Module cannot recover from an error
    ///
    /// Corrective action(s):
    ///
    /// - Main Module needs to be replaced
    E_MM79 = 50397185,
    /// E_MM80: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Open Top Cover, remove object (if any) from the inlet area, clean positioner, close cover.
    ///
    /// - Main Module needs to be replaced
    E_MM80 = 50397186,
    /// E_MM81: The Main Module cannot recover from an error
    ///
    /// Corrective action(s):
    ///
    /// - Main Module needs to be replaced
    E_MM81 = 50398977,
    /// E_MM82: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove Cashbox, remove Main module, remove object (if any) from the bottom transport area, clean bottom transport
    ///   - If problem persists: Main Module needs to be replaced
    E_MM82 = 50398978,
    /// E_MM83: The Main Module cannot recover from an error
    ///
    /// Corrective action(s):
    ///
    /// - Main Module needs to be replaced
    E_MM83 = 50399233,
    /// E_MM84: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove cashbox, remove object (if any) from the bottom bill path
    ///   - If problem persists: Main Module needs to be replaced
    E_MM84 = 50399234,
    /// E_MM87: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove object (if any) from the Outlet slot.
    ///   - If problem persists: Main Module needs to be replaced
    E_MM87 = 302055427,
    /// E_MM85: The Main Module cannot recover from an error
    ///
    /// Corrective action(s):
    ///
    /// - Main Module needs to be replaced
    E_MM85 = 50400001,
    /// E_MM86: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove cashbox, remove object (if any) from the bottom bill path
    ///   - If problem persists: Main Module needs to be replaced
    E_MM86 = 50400002,
    /// E_MM88: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Open Recognition Sensor/Spine covers. Remove unexpected object (if any).
    ///
    /// - Main Module needs to be replaced
    E_MM88 = 2818572289,
    /// E_MM89: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Open Recognition Sensor/Spine covers. Remove unexpected object (if any).
    ///
    /// - Main Module needs to be replaced
    E_MM89 = 2818572290,
    /// E_MM90: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Open Recognition Sensor/Spine covers. Remove unexpected object (if any).
    ///
    /// - Main Module needs to be replaced
    E_MM90 = 2818572291,
    /// E_MM91: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Open Recognition Module. Remove banknote.
    E_MM91 = 100663313,
    /// E_MM92: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Open Recognition Module. Remove banknote.
    E_MM92 = 100663314,
    /// E_MM93: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Open Recognition Module. Remove banknote.
    E_MM93 = 100663315,
    /// E_MM94: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Open Recognition Module. Remove banknote.
    E_MM94 = 100663316,
    /// E_BU01: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove banknotes (if any) from the Bundler.
    ///   - If problem persists: Main Module needs to be replaced
    E_BU01 = 235274305,
    /// E_BU02: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove banknotes (if any) from the Bundler.
    ///   - If problem persists: Main Module needs to be replaced
    E_BU02 = 235274306,
    /// E_BU03: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove banknotes (if any) from the Bundler.
    ///   - If problem persists: Main Module needs to be replaced
    E_BU03 = 235274307,
    /// W_BU01: Bundler is full
    ///
    /// Corrective action(s):
    ///
    /// - If in CashIn transaction, either send CashInEnd or CashInRollback; if in Dispense transaction either send Present or Reject.
    W_BU01 = 235274276,
    /// E_BU04: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove cashbox, remove Main Module, remove object (if any) from the Bundler area
    ///   - If problem persists: Main Module needs to be replaced
    E_BU04 = 16908290,
    /// E_BU05: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove cashbox, remove Main Module, remove object (if any) from the Bundler area
    ///   - If problem persists: Main Module needs to be replaced
    E_BU05 = 16908291,
    /// E_BU06: The Main Module cannot recover from an error
    ///
    /// Corrective action(s):
    ///
    /// - Main Module needs to be replaced
    E_BU06 = 50398721,
    /// E_BU07: The Main Module cannot recover from an error
    ///
    /// Corrective action(s):
    ///
    /// - Remove cashbox, remove Main Module, remove object (if any) from the Bundler area
    ///   - If problem persists: Main Module needs to be replaced
    E_BU07 = 50398722,
    /// E_BU08: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove banknotes (if any) from the Bundler.
    ///   - If problem persists: Main Module needs to be replaced
    E_BU08 = 235274257,
    /// E_BU09: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove banknotes (if any) from the Bundler.
    ///   - If problem persists: Main Module needs to be replaced
    E_BU09 = 235274258,
    /// E_BU10: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove banknotes (if any) from the Bundler.
    ///   - If problem persists: Main Module needs to be replaced
    E_BU10 = 235274259,
    /// E_BU11: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove banknotes (if any) from the Bundler.
    ///   - If problem persists: Main Module needs to be replaced
    E_BU11 = 235274260,
    /// E_SP01: BNR cannot find a module
    ///
    /// Corrective action(s):
    ///
    /// - Power off. Check connection between Spine Module and Main Module.
    ///   - If problem persists: Either Spine Module or Main Module needs to be replaced
    E_SP01 = 234946609,
    /// E_SP02: A software error has ocurred
    ///
    /// Corrective action(s):
    ///
    /// - Send BNR Configuration file to unit
    E_SP02 = 234946610,
    /// E_SP03: BNR cannot find a module
    ///
    /// Corrective action(s):
    ///
    /// - Power off. Check connection between Spine Module and Main Module
    ///   - If problem persists: Either Spine Module or Main Module needs to be replaced
    E_SP03 = 234946611,
    /// E_SP04: BNR has detected an open cover which is preventing operation
    ///
    /// Corrective action(s):
    ///
    /// - Close the spine cover
    E_SP04 = 234946612,
    /// E_SP05: BNR is not correctly configured
    ///
    /// Corrective action(s):
    ///
    /// -  
    ///
    /// - Send BNR Configuration file to unit
    E_SP05 = 234946613,
    /// E_SP06: BNR is not correctly configured
    ///
    /// Corrective action(s):
    ///
    /// -  
    ///
    /// - Send BNR Configuration file to unit
    E_SP06 = 234946614,
    /// E_SP13: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// -  Open Spine Cover. Remove banknote
    E_SP13 = 50790929,
    /// E_SP14: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// -  Open Spine Cover. Remove banknote
    E_SP14 = 50791185,
    /// E_SP15: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// -  Open Spine Cover. Remove banknote
    E_SP15 = 50791441,
    /// E_SP16: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// -  Open Spine Cover. Remove banknote
    E_SP16 = 50790930,
    /// E_SP17: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// -  Open Spine Cover. Remove banknote
    E_SP17 = 50791186,
    /// E_SP18: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// -  Open Spine Cover. Remove banknote
    E_SP18 = 50791442,
    /// E_SP19: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// -  Open Spine Cover. Remove banknote
    E_SP19 = 50790931,
    /// E_SP20: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// -  Open Spine Cover. Remove banknote
    E_SP20 = 50791187,
    /// E_SP21: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// -  Open Spine Cover. Remove banknote
    E_SP21 = 50791443,
    /// E_SP22: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// -  Open Spine Cover. Remove banknote
    E_SP22 = 50790932,
    /// E_SP23: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// -  Open Spine Cover. Remove banknote
    E_SP23 = 50791188,
    /// E_SP24: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// -  Open Spine Cover. Remove banknote
    E_SP24 = 50791444,
    /// E_SP25: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Open Spine door. remove object (if any) blocking the diverters
    ///
    /// - Spine Module needs to be replaced
    E_SP25 = 67109121,
    /// E_SP26: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Open Spine door. remove object (if any) blocking the diverters
    ///
    /// - Spine Module needs to be replaced
    E_SP26 = 67109122,
    /// E_SP27: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Open Spine door. remove object (if any) blocking the diverters
    ///
    /// - Spine Module needs to be replaced
    E_SP27 = 67109123,
    /// E_SP28: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Open Spine door. remove object (if any) blocking the diverters
    ///
    /// - Spine Module needs to be replaced
     E_SP28 = 67109377,
    /// E_SP29: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Open Spine door. remove object (if any) blocking the diverters
    ///
    /// - Spine Module needs to be replaced
    E_SP29 = 67109378,
    /// E_SP30: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Open Spine door. remove object (if any) blocking the diverters
    ///
    /// - Spine Module needs to be replaced
    E_SP30 = 67109379,
    /// E_SP31: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Open Spine door. remove object (if any) blocking the diverters
    ///
    /// - Spine Module needs to be replaced
     E_SP31 = 67109633,
    /// E_SP32: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Open Spine door. remove object (if any) blocking the diverters
    ///
    /// - Spine Module needs to be replaced
    E_SP32 = 67109634,
    /// E_SP33: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Open Spine door. remove object (if any) blocking the diverters
    ///
    /// - Spine Module needs to be replaced
    E_SP33 = 67109635,
    /// E_SP34: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Open Spine door. remove object (if any) blocking the diverters
    ///
    /// - Spine Module needs to be replaced
     E_SP34 = 67109889,
    /// E_SP35: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Open Spine door. remove object (if any) blocking the diverters
    ///
    /// - Spine Module needs to be replaced
    E_SP35 = 67109890,
    /// E_SP36: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Open Spine door. remove object (if any) blocking the diverters
    ///
    /// - Spine Module needs to be replaced
    E_SP36 = 67109891,
    /// E_SP37: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Open Spine door. remove object (if any) blocking the diverters
    ///
    /// - Spine Module needs to be replaced
     E_SP37 = 67110145,
    /// E_SP38: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Open Spine door. remove object (if any) blocking the diverters
    ///
    /// - Spine Module needs to be replaced
    E_SP38 = 67110146,
    /// E_SP39: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Open Spine door. remove object (if any) blocking the diverters
    ///
    /// - Spine Module needs to be replaced
    E_SP39 = 67110147,
    /// E_SP07: BNR cannot recover from an error
    ///
    /// Corrective action(s):
    ///
    /// - Spine Module needs to be replaced
    E_SP07 = 50790913,
    /// E_SP08: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Open Spine door. remove object (if any) in the Spine Bill path
    ///
    /// - Spine Module needs to be replaced
    E_SP08 = 50790914,
    /// E_SP09: BNR cannot recover from an error
    ///
    /// Corrective action(s):
    ///
    /// - Spine Module needs to be replaced
    E_SP09 = 50791169,
    /// E_SP10: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Open Spine door. remove object (if any) in the Spine Bill path
    ///
    /// - Spine Module needs to be replaced
     E_SP10 = 50791170,
    /// E_SP11: BNR cannot recover from an error
    ///
    /// Corrective action(s):
    ///
    /// - Spine Module needs to be replaced
     E_SP11 = 50791425,
    /// E_SP12: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Open Spine door. remove object (if any) in the Spine Bill path
    ///
    /// - Spine Module needs to be replaced
    E_SP12 = 50791426,
    /// E_SP40: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// -  Open Recognition Sensor cover or Spine Cover. Remove banknote
    E_SP40 = 234946577,
    /// E_SP41: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// -  Open Spine Cover or open module. Remove banknote
    E_SP41 = 234946581,
    /// E_SP42: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// -  Open Spine Cover. Remove banknote
    ///
    /// - Remove cashbox and Main Module. Remove banknote  from bottom of main module
    E_SP42 = 234946585,
    /// E_SP43: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// -  Open Recognition Sensor cover or Spine Cover. Remove banknote
    E_SP43 = 234946578,
    /// E_SP44: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// -  Open Spine Cover or open module. Remove banknote
    E_SP44 = 234946582,
    /// E_SP45: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// -  Open Spine Cover. Remove banknote
    ///
    /// - Remove cashbox and Main Module. Remove banknote  from bottom of main module
    E_SP45 = 234946586,
    /// E_SP46: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// -  Open Recognition Sensor cover or Spine Cover. Remove banknote
    E_SP46 = 234946579,
    /// E_SP47: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// -  Open Spine Cover or open module. Remove banknote
    E_SP47 = 234946583,
    /// E_SP48: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// -  Open Spine Cover. Remove banknote
    ///
    /// - Remove cashbox and Main Module. Remove banknote  from bottom of main module
    E_SP48 = 234946587,
    /// E_SP49: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// -  Open Recognition Sensor cover or Spine Cover. Remove banknote
    E_SP49 = 234946580,
    /// E_SP50: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// -  Open Spine Cover or open module. Remove banknote
    E_SP50 = 234946584,
    /// E_SP51: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// -  Open Spine Cover. Remove banknote
    ///
    /// - Remove cashbox and Main Module. Remove banknote  from bottom of main module
    E_SP51 = 234946588,
    /// W_LO101: BNR is not correctly configured
    ///
    /// Corrective action(s):
    ///
    /// - bnr_ConfigureCashUnit() to unlock the corresponding PhysicalCashUnit.
    ///
    /// - bnr_ConfigureCashUnit() to unlock the corresponding PhysicalCashUnit.
    W_LO101 = 235143472,
    /// E_LO101: BNR cannot find the Loader module
    ///
    /// Corrective action(s):
    ///
    /// - Loader needs to be replaced
    E_LO101 = 235143473,
    /// E_LO102: BNR is not correctly configured
    ///
    /// Corrective action(s):
    ///
    /// - Send BNR Configuration file to unit
    E_LO102 = 235143474,
    /// E_LO103: BNR cannot find the Loader module
    ///
    /// Corrective action(s):
    ///
    /// - Check Loader is correctly inserted.
    ///
    /// - Loader needs to be replaced
    E_LO103 = 235143475,
    /// E_LO104: BNR is not correctly configured
    ///
    /// Corrective action(s):
    ///
    /// - Send BNR Configuration file to unit
    E_LO104 = 235143477,
    /// E_LO105: BNR is not correctly configured
    ///
    /// Corrective action(s):
    ///
    /// - Send BNR Configuration file to unit
    E_LO105 = 235143478,
    /// W_LO102: Loader module near empty
    ///
    /// Corrective action(s):
    ///
    /// - Complete loader exchange or refill
    W_LO102 = 235143457,
    /// W_LO103: Loader module empty
    ///
    /// Corrective action(s):
    ///
    /// - Complete loader exchange or refill
    W_LO103 = 235143458,
    /// E_LO106: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Open spine cover remove Banknote.
    E_LO106 = 235143441,
    /// E_LO107: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Open spine cover remove Banknote.
    E_LO107 = 235143442,
    /// E_LO108: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Open spine cover remove Banknote.
    E_LO108 = 235143443,
    /// E_LO109: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Open spine cover remove Banknote.
    E_LO109 = 235143444,
    /// E_LO110: Loader module failure
    ///
    /// Corrective action(s):
    ///
    /// - Remove Loader. Open Loader, place bills correctly, review mechanical status of parts
    ///   - If problem persists: Loader needs to be replaced
    E_LO110 = 117453314,
    /// E_LO111: Loader module failure
    ///
    /// Corrective action(s):
    ///
    /// - Remove Loader. Open Loader, place bills correctly, review mechanical status of parts
    ///   - If problem persists: Loader needs to be replaced
    E_LO111 = 117453315,
    /// E_LO112: Loader module failure
    ///
    /// Corrective action(s):
    ///
    /// - Loader needs to be replaced
    E_LO112 = 50810881,
    /// E_LO113: Loader module failure
    ///
    /// Corrective action(s):
    ///
    /// - Remove Loader. Open Loader, review mechanical status of parts, place banknotes correctly
    ///   - If problem persists: Loader needs to be replaced
    E_LO113 = 50810882,
    /// E_LO114: Loader module failure
    ///
    /// Corrective action(s):
    ///
    /// - Remove Loader. Open Loader, review mechanical status of parts, place banknotes correctly
    ///   - If problem persists: Loader needs to be replaced
    E_LO114 = 285212930,
    /// E_LO115: Loader module failure
    ///
    /// Corrective action(s):
    ///
    /// - Remove Loader. Open Loader, review mechanical status of parts, place banknotes correctly
    ///   - If problem persists: Loader needs to be replaced
    E_LO115 = 285212931,
    /// E_LO116: Loader module failure
    ///
    /// Corrective action(s):
    ///
    /// - Remove Loader. Open Loader, review mechanical status of parts, place banknotes correctly
    ///   - If problem persists: Loader needs to be replaced
    E_LO116 = 301990147,
    /// E_LO117: Loader CashType does not match CashUnit configuration
    ///
    /// Corrective action(s):
    ///
    /// - Replace Loader with one having CashType corresponding to CashUnit configuration.
    ///
    /// - Change either Loader CashType or CashUnit configuration to have both match
    E_LO117 = 235143490,
    /// E_LO118: Bills extracted from Loader do not correspond to configured Loader CashType
    ///
    /// Corrective action(s):
    ///
    /// - Remove Loader. Replace bills with correct bills.
    E_LO118 = 235143491,
    /// E_LO119: Bills extracted from Loader are not recognized by the BNR
    ///
    /// Corrective action(s):
    ///
    /// - Remove Loader. Replace bills with bills that BNR can recognize..
    ///
    /// - Clean Recognition Sensor.
    E_LO119 = 235143492,
    /// W_LO301: BNR is not correctly configured
    ///
    /// Corrective action(s):
    ///
    /// - bnr_ConfigureCashUnit() to unlock the corresponding PhysicalCashUnit.
    ///
    /// - bnr_ConfigureCashUnit() to unlock the corresponding PhysicalCashUnit.
    W_LO301 = 235143984,
    /// E_LO301: BNR cannot find the Loader module
    ///
    /// Corrective action(s):
    ///
    /// - Loader needs to be replaced
    E_LO301 = 235143985,
    /// E_LO302: BNR is not correctly configured
    ///
    /// Corrective action(s):
    ///
    /// - Send BNR Configuration file to unit
    E_LO302 = 235143986,
    /// E_LO303: BNR cannot find the Loader module
    ///
    /// Corrective action(s):
    ///
    /// - Check Loader is correctly inserted.
    ///
    /// - Loader needs to be replaced
    E_LO303 = 235143987,
    /// E_LO304: BNR is not correctly configured
    ///
    /// Corrective action(s):
    ///
    /// - Send BNR Configuration file to unit
    E_LO304 = 235143989,
    /// E_LO305: BNR is not correctly configured
    ///
    /// Corrective action(s):
    ///
    /// - Send BNR Configuration file to unit
    E_LO305 = 235143990,
    /// W_LO302: Loader module near empty
    ///
    /// Corrective action(s):
    ///
    /// - Complete loader exchange or refill
    W_LO302 = 235143969,
    /// W_LO303: Loader module empty
    ///
    /// Corrective action(s):
    ///
    /// - Complete loader exchange or refill
    W_LO303 = 235143970,
    /// E_LO306: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Open spine cover remove Banknote.
    E_LO306 = 235143953,
    /// E_LO307: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Open spine cover remove Banknote.
    E_LO307 = 235143954,
    /// E_LO308: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Open spine cover remove Banknote.
    E_LO308 = 235143955,
    /// E_LO309: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Open spine cover remove Banknote.
    E_LO309 = 235143956,
    /// E_LO310: Loader module failure
    ///
    /// Corrective action(s):
    ///
    /// - Remove Loader. Open Loader, place bills correctly, review mechanical status of parts
    ///   - If problem persists: Loader needs to be replaced
    E_LO310 = 117453570,
    /// E_LO311: Loader module failure
    ///
    /// Corrective action(s):
    ///
    /// - Remove Loader. Open Loader, place bills correctly, review mechanical status of parts
    ///   - If problem persists: Loader needs to be replaced
    E_LO311 = 117453571,
    /// E_LO312: Loader module failure
    ///
    /// Corrective action(s):
    ///
    /// - Loader needs to be replaced
    E_LO312 = 50811137,
    /// E_LO313: Loader module failure
    ///
    /// Corrective action(s):
    ///
    /// - Remove Loader. Open Loader, review mechanical status of parts, place banknotes correctly
    ///   - If problem persists: Loader needs to be replaced
    E_LO313 = 50811138,
    /// E_LO314: Loader module failure
    ///
    /// Corrective action(s):
    ///
    /// - Remove Loader. Open Loader, review mechanical status of parts, place banknotes correctly
    ///   - If problem persists: Loader needs to be replaced
    E_LO314 = 285213186,
    /// E_LO315: Loader module failure
    ///
    /// Corrective action(s):
    ///
    /// - Remove Loader. Open Loader, review mechanical status of parts, place banknotes correctly
    ///   - If problem persists: Loader needs to be replaced
    E_LO315 = 285213187,
    /// E_LO316: Loader module failure
    ///
    /// Corrective action(s):
    ///
    /// - Remove Loader. Open Loader, review mechanical status of parts, place banknotes correctly
    ///   - If problem persists: Loader needs to be replaced
    E_LO316 = 301990403,
    /// E_LO317: Loader CashType does not match CashUnit configuration
    ///
    /// Corrective action(s):
    ///
    /// - Replace Loader with one having CashType corresponding to CashUnit configuration.
    ///
    /// - Change either Loader CashType or CashUnit configuration to have both match
    E_LO317 = 235144002,
    /// E_LO318: Bills extracted from Loader do not correspond to configured Loader CashType
    ///
    /// Corrective action(s):
    ///
    /// - Remove Loader. Replace bills with correct bills.
    E_LO318 = 235144003,
    /// E_LO319: Bills extracted from Loader are not recognized by the BNR
    ///
    /// Corrective action(s):
    ///
    /// - Remove Loader. Replace bills with bills that BNR can recognize..
    ///
    /// - Clean Recognition Sensor.
    E_LO319 = 235144004,
    /// W_LO501: BNR is not correctly configured
    ///
    /// Corrective action(s):
    ///
    /// - bnr_ConfigureCashUnit() to unlock the corresponding PhysicalCashUnit.
    ///
    /// - bnr_ConfigureCashUnit() to unlock the corresponding PhysicalCashUnit.
    W_LO501 = 235144496,
    /// E_LO501: BNR cannot find the Loader module
    ///
    /// Corrective action(s):
    ///
    /// - Loader needs to be replaced
    E_LO501 = 235144497,
    /// E_LO502: BNR is not correctly configured
    ///
    /// Corrective action(s):
    ///
    /// - Send BNR Configuration file to unit
    E_LO502 = 235144498,
    /// E_LO503: BNR cannot find the Loader module
    ///
    /// Corrective action(s):
    ///
    /// - Check Loader is correctly inserted.
    ///
    /// - Loader needs to be replaced
    E_LO503 = 235144499,
    /// E_LO504: BNR is not correctly configured
    ///
    /// Corrective action(s):
    ///
    /// - Send BNR Configuration file to unit
    E_LO504 = 235144501,
    /// E_LO505: BNR is not correctly configured
    ///
    /// Corrective action(s):
    ///
    /// - Send BNR Configuration file to unit
    E_LO505 = 235144502,
    /// W_LO502: Loader module near empty
    ///
    /// Corrective action(s):
    ///
    /// - Complete loader exchange or refill
    W_LO502 = 235144481,
    /// W_LO503: Loader module empty
    ///
    /// Corrective action(s):
    ///
    /// - Complete loader exchange or refill
    W_LO503 = 235144482,
    /// E_LO506: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Open spine cover remove Banknote.
    E_LO506 = 235144465,
    /// E_LO507: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Open spine cover remove Banknote.
    E_LO507 = 235144466,
    /// E_LO508: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Open spine cover remove Banknote.
    E_LO508 = 235144467,
    /// E_LO509: A bill has stopped before reaching its destination
    ///
    /// Corrective action(s):
    ///
    /// - Open spine cover remove Banknote.
    E_LO509 = 235144468,
    /// E_LO510: Loader module failure
    ///
    /// Corrective action(s):
    ///
    /// - Remove Loader. Open Loader, place bills correctly, review mechanical status of parts
    ///   - If problem persists: Loader needs to be replaced
    E_LO510 = 117453826,
    /// E_LO511: Loader module failure
    ///
    /// Corrective action(s):
    ///
    /// - Remove Loader. Open Loader, place bills correctly, review mechanical status of parts
    ///   - If problem persists: Loader needs to be replaced
    E_LO511 = 117453827,
    /// E_LO512: Loader module failure
    ///
    /// Corrective action(s):
    ///
    /// - Loader needs to be replaced
    E_LO512 = 50811393,
    /// E_LO513: Loader module failure
    ///
    /// Corrective action(s):
    ///
    /// - Remove Loader. Open Loader, review mechanical status of parts, place banknotes correctly
    ///   - If problem persists: Loader needs to be replaced
    E_LO513 = 50811394,
    /// E_LO514: Loader module failure
    ///
    /// Corrective action(s):
    ///
    /// - Remove Loader. Open Loader, review mechanical status of parts, place banknotes correctly
    ///   - If problem persists: Loader needs to be replaced
    E_LO514 = 285213442,
    /// E_LO515: Loader module failure
    ///
    /// Corrective action(s):
    ///
    /// - Remove Loader. Open Loader, review mechanical status of parts, place banknotes correctly
    ///   - If problem persists: Loader needs to be replaced
    E_LO515 = 285213443,
    /// E_LO516: Loader module failure
    ///
    /// Corrective action(s):
    ///
    /// - Remove Loader. Open Loader, review mechanical status of parts, place banknotes correctly
    ///   - If problem persists: Loader needs to be replaced
    E_LO516 = 301990659,
    /// E_LO517: Loader CashType does not match CashUnit configuration
    ///
    /// Corrective action(s):
    ///
    /// - Replace Loader with one having CashType corresponding to CashUnit configuration.
    ///
    /// - Change either Loader CashType or CashUnit configuration to have both match
    E_LO517 = 235144514,
    /// E_LO518: Bills extracted from Loader do not correspond to configured Loader CashType
    ///
    /// Corrective action(s):
    ///
    /// - Remove Loader. Replace bills with correct bills.
    E_LO518 = 235144515,
    /// E_LO519: Bills extracted from Loader are not recognized by the BNR
    ///
    /// Corrective action(s):
    ///
    /// - Remove Loader. Replace bills with bills that BNR can recognize..
    ///
    /// - Clean Recognition Sensor.
    E_LO519 = 235144516,
    /// W_RE101: BNR is not correctly configured
    ///
    /// Corrective action(s):
    ///
    /// - bnr_ConfigureCashUnit() to unlock the corresponding PhysicalCashUnit.
    ///
    /// - bnr_ConfigureCashUnit() to unlock the corresponding PhysicalCashUnit.
    W_RE101 = 235077936,
    /// E_RE101: BNR cannot find a Recycler module
    ///
    /// Corrective action(s):
    ///
    /// - Recycler needs to be replaced
    E_RE101 = 235077937,
    /// E_RE102: BNR is not correctly configured
    ///
    /// Corrective action(s):
    ///
    /// - Send BNR Configuration file to unit
    E_RE102 = 235077938,
    /// E_RE103: BNR cannot find a Recycler module
    ///
    /// Corrective action(s):
    ///
    /// - Check if Recycler  is correctly inserted.
    ///   - If problem persists: Recycler needs to be replaced
    E_RE103 = 235077939,
    /// E_RE104: BNR is not correctly configured
    ///
    /// Corrective action(s):
    ///
    /// - Send BNR Configuration file to unit
    E_RE104 = 235077941,
    /// E_RE105: BNR is not correctly configured
    ///
    /// Corrective action(s):
    ///
    /// - Send BNR Configuration file to unit
    E_RE105 = 235077942,
    /// W_RE102: A Recycler module is near empty
    ///
    /// Corrective action(s):
    ///
    /// - If this is a key banknote, prep to add banknotes to the BNR
    W_RE102 = 235077921,
    /// W_RE103: A Recycler module is empty
    ///
    /// Corrective action(s):
    ///
    /// - If this is a key banknote, add more of that banknote to the BNR
    W_RE103 = 235077922,
    /// W_RE104: 
    ///
    /// Corrective action(s):
    W_RE104 = 235077923,
    /// W_RE105: 
    ///
    /// Corrective action(s):
    W_RE105 = 235077924,
    /// E_RE106: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove recycler, Remove banknote from spine or recycler
    E_RE106 = 235077905,
    /// E_RE107: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove recycler, Remove banknote from spine or recycler
    E_RE107 = 235077906,
    /// E_RE108: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove recycler, Remove banknote from spine or recycler
    E_RE108 = 235077907,
    /// E_RE109: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove recycler, Remove banknote from spine or recycler
    E_RE109 = 235077908,
    /// E_RE110: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove Recycler, review mechanical status of parts, remove object (if any) blocking the tape. secure BNR.
    ///   - If problem persists: Recycler needs to be replaced
    E_RE110 = 117440770,
    /// E_RE111: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove Recycler, review mechanical status of parts, remove object (if any) blocking the tape. secure BNR.
    ///   - If problem persists: Recycler needs to be replaced
    E_RE111 = 117440771,
    /// E_RE112: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Recycler needs to be replaced
    E_RE112 = 50803201,
    /// E_RE113: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove Recycler. Remove object (if any) in the Recycler bill path
    ///   - If problem persists: Recycler Module needs to be replaced
    E_RE113 = 50803202,
    /// E_RE114: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove Recycler. Remove object (if any) on the coding wheel sensors.
    ///   - If problem persists: Recycler needs to be replaced
    E_RE114 = 218104066,
    /// E_RE115: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove Recycler. Remove object (if any) on the coding wheel sensors.
    ///   - If problem persists: Recycler needs to be replaced
    E_RE115 = 218104067,
    /// E_RE116: Recycler CashType does not match CashUnit configuration
    ///
    /// Corrective action(s):
    ///
    /// - Empty Recycler, or change CashUnit configuration to have it match Recycler CashType.
    E_RE116 = 235077954,
    /// W_RE201: BNR is not correctly configured
    ///
    /// Corrective action(s):
    ///
    /// - bnr_ConfigureCashUnit() to unlock the corresponding PhysicalCashUnit.
    ///
    /// - bnr_ConfigureCashUnit() to unlock the corresponding PhysicalCashUnit.
    W_RE201 = 235078192,
    /// E_RE201: BNR cannot find a Recycler module
    ///
    /// Corrective action(s):
    ///
    /// - Recycler needs to be replaced
    E_RE201 = 235078193,
    /// E_RE202: BNR is not correctly configured
    ///
    /// Corrective action(s):
    ///
    /// - Send BNR Configuration file to unit
    E_RE202 = 235078194,
    /// E_RE203: BNR cannot find a Recycler module
    ///
    /// Corrective action(s):
    ///
    /// - Check if Recycler  is correctly inserted.
    ///   - If problem persists: Recycler needs to be replaced
    E_RE203 = 235078195,
    /// E_RE204: BNR is not correctly configured
    ///
    /// Corrective action(s):
    ///
    /// - Send BNR Configuration file to unit
    E_RE204 = 235078197,
    /// E_RE205: BNR is not correctly configured
    ///
    /// Corrective action(s):
    ///
    /// - Send BNR Configuration file to unit
    E_RE205 = 235078198,
    /// W_RE202: A Recycler module is near empty
    ///
    /// Corrective action(s):
    ///
    /// - If this is a key banknote, prep to add banknotes to the BNR
    W_RE202 = 235078177,
    /// W_RE203: A Recycler module is empty
    ///
    /// Corrective action(s):
    ///
    /// - If this is a key banknote, add more of that banknote to the BNR
    W_RE203 = 235078178,
    /// W_RE204: 
    ///
    /// Corrective action(s):
    W_RE204 = 235078179,
    /// W_RE205: 
    ///
    /// Corrective action(s):
    W_RE205 = 235078180,
    /// E_RE206: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove recycler, Remove banknote from spine or recycler
    E_RE206 = 235078161,
    /// E_RE207: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove recycler, Remove banknote from spine or recycler
    E_RE207 = 235078162,
    /// E_RE208: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove recycler, Remove banknote from spine or recycler
    E_RE208 = 235078163,
    /// E_RE209: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove recycler, Remove banknote from spine or recycler
    E_RE209 = 235078164,
    /// E_RE210: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove Recycler, review mechanical status of parts, remove object (if any) blocking the tape. secure BNR.
    ///   - If problem persists: Recycler needs to be replaced
    E_RE210 = 117441026,
    /// E_RE211: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove Recycler, review mechanical status of parts, remove object (if any) blocking the tape. secure BNR.
    ///   - If problem persists: Recycler needs to be replaced
    E_RE211 = 117441027,
    /// E_RE212: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Recycler needs to be replaced
    E_RE212 = 50803457,
    /// E_RE213: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove Recycler. Remove object (if any) in the Recycler bill path
    ///   - If problem persists: Recycler Module needs to be replaced
    E_RE213 = 50803458,
    /// E_RE214: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove Recycler. Remove object (if any) on the coding wheel sensors.
    ///   - If problem persists: Recycler needs to be replaced
    E_RE214 = 218104322,
    /// E_RE215: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove Recycler. Remove object (if any) on the coding wheel sensors.
    ///   - If problem persists: Recycler needs to be replaced
    E_RE215 = 218104323,
    /// E_RE216: Recycler CashType does not match CashUnit configuration
    ///
    /// Corrective action(s):
    ///
    /// - Empty Recycler, or change CashUnit configuration to have it match Recycler CashType.
    E_RE216 = 235078210,
    /// W_RE301: BNR is not correctly configured
    ///
    /// Corrective action(s):
    ///
    /// - bnr_ConfigureCashUnit() to unlock the corresponding PhysicalCashUnit.
    ///
    /// - bnr_ConfigureCashUnit() to unlock the corresponding PhysicalCashUnit.
    W_RE301 = 235078448,
    /// E_RE301: BNR cannot find a Recycler module
    ///
    /// Corrective action(s):
    ///
    /// - Recycler needs to be replaced
    E_RE301 = 235078449,
    /// E_RE302: BNR is not correctly configured
    ///
    /// Corrective action(s):
    ///
    /// - Send BNR Configuration file to unit
    E_RE302 = 235078450,
    /// E_RE303: BNR cannot find a Recycler module
    ///
    /// Corrective action(s):
    ///
    /// - Check if Recycler  is correctly inserted.
    ///   - If problem persists: Recycler needs to be replaced
    E_RE303 = 235078451,
    /// E_RE304: BNR is not correctly configured
    ///
    /// Corrective action(s):
    ///
    /// - Send BNR Configuration file to unit
    E_RE304 = 235078453,
    /// E_RE305: BNR is not correctly configured
    ///
    /// Corrective action(s):
    ///
    /// - Send BNR Configuration file to unit
    E_RE305 = 235078454,
    /// W_RE302: A Recycler module is near empty
    ///
    /// Corrective action(s):
    ///
    /// - If this is a key banknote, prep to add banknotes to the BNR
    W_RE302 = 235078433,
    /// W_RE303: A Recycler module is empty
    ///
    /// Corrective action(s):
    ///
    /// - If this is a key banknote, add more of that banknote to the BNR
    W_RE303 = 235078434,
    /// W_RE304: 
    ///
    /// Corrective action(s):
    W_RE304 = 235078435,
    /// W_RE305: 
    ///
    /// Corrective action(s):
    W_RE305 = 235078436,
    /// E_RE306: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove recycler, Remove banknote from spine or recycler
    E_RE306 = 235078417,
    /// E_RE307: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove recycler, Remove banknote from spine or recycler
    E_RE307 = 235078418,
    /// E_RE308: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove recycler, Remove banknote from spine or recycler
    E_RE308 = 235078419,
    /// E_RE309: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove recycler, Remove banknote from spine or recycler
    E_RE309 = 235078420,
    /// E_RE310: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove Recycler, review mechanical status of parts, remove object (if any) blocking the tape. secure BNR.
    ///   - If problem persists: Recycler needs to be replaced
    E_RE310 = 117441282,
    /// E_RE311: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove Recycler, review mechanical status of parts, remove object (if any) blocking the tape. secure BNR.
    ///   - If problem persists: Recycler needs to be replaced
    E_RE311 = 117441283,
    /// E_RE312: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Recycler needs to be replaced
    E_RE312 = 50803713,
    /// E_RE313: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove Recycler. Remove object (if any) in the Recycler bill path
    ///   - If problem persists: Recycler Module needs to be replaced
    E_RE313 = 50803714,
    /// E_RE314: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove Recycler. Remove object (if any) on the coding wheel sensors.
    ///   - If problem persists: Recycler needs to be replaced
    E_RE314 = 218104578,
    /// E_RE315: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove Recycler. Remove object (if any) on the coding wheel sensors.
    ///   - If problem persists: Recycler needs to be replaced
    E_RE315 = 218104579,
    /// E_RE316: Recycler CashType does not match CashUnit configuration
    ///
    /// Corrective action(s):
    ///
    /// - Empty Recycler, or change CashUnit configuration to have it match Recycler CashType.
    E_RE316 = 235078466,
    /// W_RE401: BNR is not correctly configured
    ///
    /// Corrective action(s):
    ///
    /// - bnr_ConfigureCashUnit() to unlock the corresponding PhysicalCashUnit.
    ///
    /// - bnr_ConfigureCashUnit() to unlock the corresponding PhysicalCashUnit.
    W_RE401 = 235078704,
    /// E_RE401: BNR cannot find a Recycler module
    ///
    /// Corrective action(s):
    ///
    /// - Recycler needs to be replaced
    E_RE401 = 235078705,
    /// E_RE402: BNR is not correctly configured
    ///
    /// Corrective action(s):
    ///
    /// - Send BNR Configuration file to unit
    E_RE402 = 235078706,
    /// E_RE403: BNR cannot find a Recycler module
    ///
    /// Corrective action(s):
    ///
    /// - Check if Recycler  is correctly inserted.
    ///   - If problem persists: Recycler needs to be replaced
    E_RE403 = 235078707,
    /// E_RE404: BNR is not correctly configured
    ///
    /// Corrective action(s):
    ///
    /// - Send BNR Configuration file to unit
    E_RE404 = 235078709,
    /// E_RE405: BNR is not correctly configured
    ///
    /// Corrective action(s):
    ///
    /// - Send BNR Configuration file to unit
    E_RE405 = 235078710,
    /// W_RE402: A Recycler module is near empty
    ///
    /// Corrective action(s):
    ///
    /// - If this is a key banknote, prep to add banknotes to the BNR
    W_RE402 = 235078689,
    /// W_RE403: A Recycler module is empty
    ///
    /// Corrective action(s):
    ///
    /// - If this is a key banknote, add more of that banknote to the BNR
    W_RE403 = 235078690,
    /// W_RE404: 
    ///
    /// Corrective action(s):
    W_RE404 = 235078691,
    /// W_RE405: 
    ///
    /// Corrective action(s):
    W_RE405 = 235078692,
    /// E_RE406: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove recycler, Remove banknote from spine or recycler
    E_RE406 = 235078673,
    /// E_RE407: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove recycler, Remove banknote from spine or recycler
    E_RE407 = 235078674,
    /// E_RE408: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove recycler, Remove banknote from spine or recycler
    E_RE408 = 235078675,
    /// E_RE409: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove recycler, Remove banknote from spine or recycler
    E_RE409 = 235078676,
    /// E_RE410: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove Recycler, review mechanical status of parts, remove object (if any) blocking the tape. secure BNR.
    ///   - If problem persists: Recycler needs to be replaced
    E_RE410 = 117441538,
    /// E_RE411: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove Recycler, review mechanical status of parts, remove object (if any) blocking the tape. secure BNR.
    ///   - If problem persists: Recycler needs to be replaced
    E_RE411 = 117441539,
    /// E_RE412: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Recycler needs to be replaced
    E_RE412 = 50803969,
    /// E_RE413: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove Recycler. Remove object (if any) in the Recycler bill path
    ///   - If problem persists: Recycler Module needs to be replaced
    E_RE413 = 50803970,
    /// E_RE414: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove Recycler. Remove object (if any) on the coding wheel sensors.
    ///   - If problem persists: Recycler needs to be replaced
    E_RE414 = 218104834,
    /// E_RE415: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove Recycler. Remove object (if any) on the coding wheel sensors.
    ///   - If problem persists: Recycler needs to be replaced
    E_RE415 = 218104835,
    /// E_RE416: Recycler CashType does not match CashUnit configuration
    ///
    /// Corrective action(s):
    ///
    /// - Empty Recycler, or change CashUnit configuration to have it match Recycler CashType.
    E_RE416 = 235078722,
    /// W_RE501: BNR is not correctly configured
    ///
    /// Corrective action(s):
    ///
    /// - bnr_ConfigureCashUnit() to unlock the corresponding PhysicalCashUnit.
    ///
    /// - bnr_ConfigureCashUnit() to unlock the corresponding PhysicalCashUnit.
    W_RE501 = 235078960,
    /// E_RE501: BNR cannot find a Recycler module
    ///
    /// Corrective action(s):
    ///
    /// - Recycler needs to be replaced
    E_RE501 = 235078961,
    /// E_RE502: BNR is not correctly configured
    ///
    /// Corrective action(s):
    ///
    /// - Send BNR Configuration file to unit
    E_RE502 = 235078962,
    /// E_RE503: BNR cannot find a Recycler module
    ///
    /// Corrective action(s):
    ///
    /// - Check if Recycler  is correctly inserted.
    ///   - If problem persists: Recycler needs to be replaced
    E_RE503 = 235078963,
    /// E_RE504: BNR is not correctly configured
    ///
    /// Corrective action(s):
    ///
    /// - Send BNR Configuration file to unit
    E_RE504 = 235078965,
    /// E_RE505: BNR is not correctly configured
    ///
    /// Corrective action(s):
    ///
    /// - Send BNR Configuration file to unit
    E_RE505 = 235078966,
    /// W_RE502: A Recycler module is near empty
    ///
    /// Corrective action(s):
    ///
    /// - If this is a key banknote, prep to add banknotes to the BNR
    W_RE502 = 235078945,
    /// W_RE503: A Recycler module is empty
    ///
    /// Corrective action(s):
    ///
    /// - If this is a key banknote, add more of that banknote to the BNR
    W_RE503 = 235078946,
    /// W_RE504: 
    ///
    /// Corrective action(s):
    W_RE504 = 235078947,
    /// W_RE505: 
    ///
    /// Corrective action(s):
    W_RE505 = 235078948,
    /// E_RE506: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove recycler, Remove banknote from spine or recycler
    E_RE506 = 235078929,
    /// E_RE507: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove recycler, Remove banknote from spine or recycler
    E_RE507 = 235078930,
    /// E_RE508: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove recycler, Remove banknote from spine or recycler
    E_RE508 = 235078931,
    /// E_RE509: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove recycler, Remove banknote from spine or recycler
    E_RE509 = 235078932,
    /// E_RE510: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove Recycler, review mechanical status of parts, remove object (if any) blocking the tape. secure BNR.
    ///   - If problem persists: Recycler needs to be replaced
    E_RE510 = 117441794,
    /// E_RE511: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove Recycler, review mechanical status of parts, remove object (if any) blocking the tape. secure BNR.
    ///   - If problem persists: Recycler needs to be replaced
    E_RE511 = 117441795,
    /// E_RE512: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Recycler needs to be replaced
    E_RE512 = 50804225,
    /// E_RE513: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove Recycler. Remove object (if any) in the Recycler bill path
    ///   - If problem persists: Recycler Module needs to be replaced
    E_RE513 = 50804226,
    /// E_RE514: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove Recycler. Remove object (if any) on the coding wheel sensors.
    ///   - If problem persists: Recycler needs to be replaced
    E_RE514 = 218105090,
    /// E_RE515: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove Recycler. Remove object (if any) on the coding wheel sensors.
    ///   - If problem persists: Recycler needs to be replaced
    E_RE515 = 218105091,
    /// E_RE516: Recycler CashType does not match CashUnit configuration
    ///
    /// Corrective action(s):
    ///
    /// - Empty Recycler, or change CashUnit configuration to have it match Recycler CashType.
    E_RE516 = 235078978,
    /// W_RE601: BNR is not correctly configured
    ///
    /// Corrective action(s):
    ///
    /// - bnr_ConfigureCashUnit() to unlock the corresponding PhysicalCashUnit.
    ///
    /// - bnr_ConfigureCashUnit() to unlock the corresponding PhysicalCashUnit.
    W_RE601 = 235079216,
    /// E_RE601: BNR cannot find a Recycler module
    ///
    /// Corrective action(s):
    ///
    /// - Recycler needs to be replaced
    E_RE601 = 235079217,
    /// E_RE602: BNR is not correctly configured
    ///
    /// Corrective action(s):
    ///
    /// - Send BNR Configuration file to unit
    E_RE602 = 235079218,
    /// E_RE603: BNR cannot find a Recycler module
    ///
    /// Corrective action(s):
    ///
    /// - Check if Recycler  is correctly inserted.
    ///   - If problem persists: Recycler needs to be replaced
    E_RE603 = 235079219,
    /// E_RE604: BNR is not correctly configured
    ///
    /// Corrective action(s):
    ///
    /// - Send BNR Configuration file to unit
    E_RE604 = 235079221,
    /// E_RE605: BNR is not correctly configured
    ///
    /// Corrective action(s):
    ///
    /// - Send BNR Configuration file to unit
    E_RE605 = 235079222,
    /// W_RE602: A Recycler module is near empty
    ///
    /// Corrective action(s):
    ///
    /// - If this is a key banknote, prep to add banknotes to the BNR
    W_RE602 = 235079201,
    /// W_RE603: A Recycler module is empty
    ///
    /// Corrective action(s):
    ///
    /// - If this is a key banknote, add more of that banknote to the BNR
    W_RE603 = 235079202,
    /// W_RE604: 
    ///
    /// Corrective action(s):
    W_RE604 = 235079203,
    /// W_RE605: 
    ///
    /// Corrective action(s):
    W_RE605 = 235079204,
    /// E_RE606: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove recycler, Remove banknote from spine or recycler
    E_RE606 = 235079185,
    /// E_RE607: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove recycler, Remove banknote from spine or recycler
    E_RE607 = 235079186,
    /// E_RE608: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove recycler, Remove banknote from spine or recycler
    E_RE608 = 235079187,
    /// E_RE609: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove recycler, Remove banknote from spine or recycler
    E_RE609 = 235079188,
    /// E_RE610: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove Recycler, review mechanical status of parts, remove object (if any) blocking the tape. secure BNR.
    ///   - If problem persists: Recycler needs to be replaced
    E_RE610 = 117442050,
    /// E_RE611: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove Recycler, review mechanical status of parts, remove object (if any) blocking the tape. secure BNR.
    ///   - If problem persists: Recycler needs to be replaced
    E_RE611 = 117442051,
    /// E_RE612: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Recycler needs to be replaced
    E_RE612 = 50804481,
    /// E_RE613: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove Recycler. Remove object (if any) in the Recycler bill path
    ///   - If problem persists: Recycler Module needs to be replaced
    E_RE613 = 50804482,
    /// E_RE614: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove Recycler. Remove object (if any) on the coding wheel sensors.
    ///   - If problem persists: Recycler needs to be replaced
    E_RE614 = 218105346,
    /// E_RE615: BNR has detected a blockage or failure in a module
    ///
    /// Corrective action(s):
    ///
    /// - Remove Recycler. Remove object (if any) on the coding wheel sensors.
    ///   - If problem persists: Recycler needs to be replaced
    E_RE615 = 218105347,
    /// E_RE616: Recycler CashType does not match CashUnit configuration
    ///
    /// Corrective action(s):
    ///
    /// - Empty Recycler, or change CashUnit configuration to have it match Recycler CashType.
    E_RE616 = 235079234,
    /// E_CB01: BNR cannot find a Cashbox module
    ///
    /// Corrective action(s):
    ///
    /// - Cashbox needs to be replaced for repair
    ///
    /// - Send BNR Configuration file to unit
    E_CB01 = 235012145,
    /// E_CB02: BNR is not correctly configured
    ///
    /// Corrective action(s):
    ///
    /// - Send BNR Configuration file to unit
    E_CB02 = 235012146,
    /// E_CB03: BNR cannot find a Cashbox module
    ///
    /// Corrective action(s):
    ///
    /// - Check module is correctly inserted.
    ///
    /// - Cashbox needs to be replaced for repair
    E_CB03 = 235012147,
    /// E_CB04: BNR is not correctly configured
    ///
    /// Corrective action(s):
    ///
    /// - Send BNR Configuration file to unit
    E_CB04 = 235012149,
    /// E_CB05: BNR is not correctly configured
    ///
    /// Corrective action(s):
    ///
    /// - Send BNR Configuration file to unit
    E_CB05 = 235012150,
    /// E_CB06: BNR Interlock is open
    ///
    /// Corrective action(s):
    ///
    /// - Close the Main BNR Lock
    E_CB06 = 235012153,
    /// E_CB07: Cashbox is not armed
    ///
    /// Corrective action(s):
    ///
    /// - Remove Cashbox and rearm Cashbox
    E_CB07 = 235012154,
    /// E_CB08: BNR is not correctly configured
    ///
    /// Corrective action(s):
    ///
    /// - Send BNR Configuration file to unit
    E_CB08 = 235012155,
    /// W_CB01: Cashbox is almost full
    ///
    /// Corrective action(s):
    ///
    /// - Cashbox almost full
    W_CB01 = 235012131,
    /// E_CB09: Cashbox is full
    ///
    /// Corrective action(s):
    ///
    /// - Field Cash Re-filler: complete Cashbox exchange
    E_CB09 = 235012132,
}

impl From<BnrError> for &'static str {
    fn from(err: BnrError) -> Self {
        match err {
            BnrError::E_SS01 => "E_SS01: BNR is idle mode",
            BnrError::E_SS02 => "E_SS02: BNR is starting up",
            BnrError::E_SS03 => "E_SS03: BNR Interlock is open",
            BnrError::E_MM01 => "E_MM01: A software error has ocurred",
            BnrError::E_MM03 => "E_MM03: A software error has ocurred",
            BnrError::E_MM04 => "E_MM04: A bill has stopped before reaching its destination",
            BnrError::E_MM05 => "E_MM05: A bill has stopped before reaching its destination",
            BnrError::E_MM06 => "E_MM06: A bill has stopped before reaching its destination",
            BnrError::E_MM07 => "E_MM07: A bill has stopped before reaching its destination",
            BnrError::E_MM08 => "E_MM08: A bill has stopped before reaching its destination",
            BnrError::E_MM09 => "E_MM09: A bill has stopped before reaching its destination",
            BnrError::E_MM10 => "E_MM10: A bill has stopped before reaching its destination",
            BnrError::E_MM11 => "E_MM11: A bill has stopped before reaching its destination",
            BnrError::E_MM12 => "E_MM12: A bill has stopped before reaching its destination",
            BnrError::E_MM13 => "E_MM13: A bill has stopped before reaching its destination",
            BnrError::E_MM14 => "E_MM14: A bill has stopped before reaching its destination",
            BnrError::E_MM15 => "E_MM15: A bill has stopped before reaching its destination",
            BnrError::E_MM16 => "E_MM16: A bill has stopped before reaching its destination",
            BnrError::E_MM17 => "E_MM17: A bill has stopped before reaching its destination",
            BnrError::E_MM18 => "E_MM18: A bill has stopped before reaching its destination",
            BnrError::E_MM19 => "E_MM19: A bill has stopped before reaching its destination",
            BnrError::E_MM20 => "E_MM20: A bill has stopped before reaching its destination",
            BnrError::E_MM21 => "E_MM21: A bill has stopped before reaching its destination",
            BnrError::E_MM22 => "E_MM22: A bill has stopped before reaching its destination",
            BnrError::E_MM23 => "E_MM23: A bill has stopped before reaching its destination",
            BnrError::E_MM24 => "E_MM24: A bill has stopped before reaching its destination",
            BnrError::E_MM25 => "E_MM25: A bill has stopped before reaching its destination",
            BnrError::E_MM26 => "E_MM26: A bill has stopped before reaching its destination",
            BnrError::E_MM27 => "E_MM27: A bill has stopped before reaching its destination",
            BnrError::E_MM28 => "E_MM28: A bill has stopped before reaching its destination",
            BnrError::E_MM29 => "E_MM29: A bill has stopped before reaching its destination",
            BnrError::E_MM30 => "E_MM30: A bill has stopped before reaching its destination",
            BnrError::E_MM31 => "E_MM31: A bill has stopped before reaching its destination",
            BnrError::E_MM32 => "E_MM32: A bill has stopped before reaching its destination",
            BnrError::E_MM33 => "E_MM33: A bill has stopped before reaching its destination",
            BnrError::E_MM34 => "E_MM34: A bill has stopped before reaching its destination",
            BnrError::E_MM35 => "E_MM35: A bill has stopped before reaching its destination",
            BnrError::E_MM41 => "E_MM41: BNR has detected a blockage or failure in a module",
            BnrError::E_MM42 => "E_MM42: BNR has detected a blockage or failure in a module",
            BnrError::E_MM43 => "E_MM43: BNR has detected a blockage or failure in a module",
            BnrError::E_MM44 => "E_MM44: BNR has detected a blockage or failure in a module",
            BnrError::E_MM45 => "E_MM45: BNR has detected a blockage or failure in a module",
            BnrError::E_MM46 => "E_MM46: BNR has detected a blockage or failure in a module",
            BnrError::E_MM47 => "E_MM47: BNR has detected a blockage or failure in a module",
            BnrError::E_MM48 => "E_MM48: BNR has detected a blockage or failure in a module",
            BnrError::E_MM49 => "E_MM49: BNR has detected a blockage or failure in a module",
            BnrError::E_MM50 => "E_MM50: BNR has detected a blockage or failure in a module",
            BnrError::E_MM51 => "E_MM51: A bill has stopped before reaching its destination",
            BnrError::E_MM52 => "E_MM52: BNR has detected a blockage or failure in a module",
            BnrError::E_MM53 => "E_MM53: BNR has detected a blockage or failure in a module",
            BnrError::E_MM54 => "E_MM54: BNR has detected a blockage or failure in a module",
            BnrError::E_MM55 => "E_MM55: BNR has detected a blockage or failure in a module",
            BnrError::E_MM56 => "E_MM56: BNR has detected a blockage or failure in a module",
            BnrError::E_MM57 => "E_MM57: BNR has detected a blockage or failure in a module",
            BnrError::E_MM58 => "E_MM58: The Main Module cannot recover from an error",
            BnrError::E_MM59 => "E_MM59: The Main Module cannot recover from an error",
            BnrError::E_MM60 => "E_MM60: The Main Module cannot recover from an error",
            BnrError::E_MM61 => "E_MM61: The Main Module cannot recover from an error",
            BnrError::E_MM62 => "E_MM62: The Main Module cannot recover from an error",
            BnrError::E_MM63 => "E_MM63: The Main Module cannot recover from an error",
            BnrError::E_MM64 => "E_MM64: The Main Module cannot recover from an error",
            BnrError::E_MM65 => "E_MM65: The Main Module cannot recover from an error",
            BnrError::E_MM66 => "E_MM66: BNR has detected a blockage or failure in a module",
            BnrError::E_MM39 => "E_MM39: BNR has detected an open cover which is preventing operation",
            BnrError::E_MM40 => "E_MM40: BNR has detected an open cover which is preventing operation",
            BnrError::E_MM67 => "E_MM67: BNR has detected a blockage or failure in a module",
            BnrError::E_MM68 => "E_MM68: BNR has detected a blockage or failure in a module",
            BnrError::E_MM69 => "E_MM69: BNR has detected a blockage or failure in a module",
            BnrError::E_MM70 => "E_MM70: BNR has detected a blockage or failure in a module",
            BnrError::E_MM71 => "E_MM71: BNR has detected a blockage or failure in a module",
            BnrError::E_MM72 => "E_MM72: BNR has detected a blockage or failure in a module",
            BnrError::E_MM73 => "E_MM73: The Main Module cannot recover from an error",
            BnrError::E_MM74 => "E_MM74: BNR has detected a blockage or failure in a module",
            BnrError::E_MM75 => "E_MM75: The Main Module cannot recover from an error",
            BnrError::E_MM76 => "E_MM76: BNR has detected a blockage or failure in a module",
            BnrError::E_MM77 => "E_MM77: The Main Module cannot recover from an error",
            BnrError::E_MM78 => "E_MM78: BNR has detected a blockage or failure in a module",
            BnrError::E_MM79 => "E_MM79: The Main Module cannot recover from an error",
            BnrError::E_MM80 => "E_MM80: BNR has detected a blockage or failure in a module",
            BnrError::E_MM81 => "E_MM81: The Main Module cannot recover from an error",
            BnrError::E_MM82 => "E_MM82: BNR has detected a blockage or failure in a module",
            BnrError::E_MM83 => "E_MM83: The Main Module cannot recover from an error",
            BnrError::E_MM84 => "E_MM84: BNR has detected a blockage or failure in a module",
            BnrError::E_MM87 => "E_MM87: BNR has detected a blockage or failure in a module",
            BnrError::E_MM85 => "E_MM85: The Main Module cannot recover from an error",
            BnrError::E_MM86 => "E_MM86: BNR has detected a blockage or failure in a module",
            BnrError::E_MM88 => "E_MM88: BNR has detected a blockage or failure in a module",
            BnrError::E_MM89 => "E_MM89: BNR has detected a blockage or failure in a module",
            BnrError::E_MM90 => "E_MM90: BNR has detected a blockage or failure in a module",
            BnrError::E_MM91 => "E_MM91: A bill has stopped before reaching its destination",
            BnrError::E_MM92 => "E_MM92: A bill has stopped before reaching its destination",
            BnrError::E_MM93 => "E_MM93: A bill has stopped before reaching its destination",
            BnrError::E_MM94 => "E_MM94: A bill has stopped before reaching its destination",
            BnrError::E_BU01 => "E_BU01: BNR has detected a blockage or failure in a module",
            BnrError::E_BU02 => "E_BU02: BNR has detected a blockage or failure in a module",
            BnrError::E_BU03 => "E_BU03: BNR has detected a blockage or failure in a module",
            BnrError::W_BU01 => "W_BU01: Bundler is full",
            BnrError::E_BU04 => "E_BU04: BNR has detected a blockage or failure in a module",
            BnrError::E_BU05 => "E_BU05: BNR has detected a blockage or failure in a module",
            BnrError::E_BU06 => "E_BU06: The Main Module cannot recover from an error",
            BnrError::E_BU07 => "E_BU07: The Main Module cannot recover from an error",
            BnrError::E_BU08 => "E_BU08: BNR has detected a blockage or failure in a module",
            BnrError::E_BU09 => "E_BU09: BNR has detected a blockage or failure in a module",
            BnrError::E_BU10 => "E_BU10: BNR has detected a blockage or failure in a module",
            BnrError::E_BU11 => "E_BU11: BNR has detected a blockage or failure in a module",
            BnrError::E_SP01 => "E_SP01: BNR cannot find a module",
            BnrError::E_SP02 => "E_SP02: A software error has ocurred",
            BnrError::E_SP03 => "E_SP03: BNR cannot find a module",
            BnrError::E_SP04 => "E_SP04: BNR has detected an open cover which is preventing operation",
            BnrError::E_SP05 => "E_SP05: BNR is not correctly configured",
            BnrError::E_SP06 => "E_SP06: BNR is not correctly configured",
            BnrError::E_SP13 => "E_SP13: BNR has detected a blockage or failure in a module",
            BnrError::E_SP14 => "E_SP14: BNR has detected a blockage or failure in a module",
            BnrError::E_SP15 => "E_SP15: BNR has detected a blockage or failure in a module",
            BnrError::E_SP16 => "E_SP16: BNR has detected a blockage or failure in a module",
            BnrError::E_SP17 => "E_SP17: BNR has detected a blockage or failure in a module",
            BnrError::E_SP18 => "E_SP18: BNR has detected a blockage or failure in a module",
            BnrError::E_SP19 => "E_SP19: BNR has detected a blockage or failure in a module",
            BnrError::E_SP20 => "E_SP20: BNR has detected a blockage or failure in a module",
            BnrError::E_SP21 => "E_SP21: BNR has detected a blockage or failure in a module",
            BnrError::E_SP22 => "E_SP22: BNR has detected a blockage or failure in a module",
            BnrError::E_SP23 => "E_SP23: BNR has detected a blockage or failure in a module",
            BnrError::E_SP24 => "E_SP24: BNR has detected a blockage or failure in a module",
            BnrError::E_SP25 => "E_SP25: BNR has detected a blockage or failure in a module",
            BnrError::E_SP26 => "E_SP26: BNR has detected a blockage or failure in a module",
            BnrError::E_SP27 => "E_SP27: BNR has detected a blockage or failure in a module",
            BnrError::E_SP28 => "E_SP28: BNR has detected a blockage or failure in a module",
            BnrError::E_SP29 => "E_SP29: BNR has detected a blockage or failure in a module",
            BnrError::E_SP30 => "E_SP30: BNR has detected a blockage or failure in a module",
            BnrError::E_SP31 => "E_SP31: BNR has detected a blockage or failure in a module",
            BnrError::E_SP32 => "E_SP32: BNR has detected a blockage or failure in a module",
            BnrError::E_SP33 => "E_SP33: BNR has detected a blockage or failure in a module",
            BnrError::E_SP34 => "E_SP34: BNR has detected a blockage or failure in a module",
            BnrError::E_SP35 => "E_SP35: BNR has detected a blockage or failure in a module",
            BnrError::E_SP36 => "E_SP36: BNR has detected a blockage or failure in a module",
            BnrError::E_SP37 => "E_SP37: BNR has detected a blockage or failure in a module",
            BnrError::E_SP38 => "E_SP38: BNR has detected a blockage or failure in a module",
            BnrError::E_SP39 => "E_SP39: BNR has detected a blockage or failure in a module",
            BnrError::E_SP07 => "E_SP07: BNR cannot recover from an error",
            BnrError::E_SP08 => "E_SP08: BNR has detected a blockage or failure in a module",
            BnrError::E_SP09 => "E_SP09: BNR cannot recover from an error",
            BnrError::E_SP10 => "E_SP10: BNR has detected a blockage or failure in a module",
            BnrError::E_SP11 => "E_SP11: BNR cannot recover from an error",
            BnrError::E_SP12 => "E_SP12: BNR has detected a blockage or failure in a module",
            BnrError::E_SP40 => "E_SP40: BNR has detected a blockage or failure in a module",
            BnrError::E_SP41 => "E_SP41: BNR has detected a blockage or failure in a module",
            BnrError::E_SP42 => "E_SP42: BNR has detected a blockage or failure in a module",
            BnrError::E_SP43 => "E_SP43: BNR has detected a blockage or failure in a module",
            BnrError::E_SP44 => "E_SP44: BNR has detected a blockage or failure in a module",
            BnrError::E_SP45 => "E_SP45: BNR has detected a blockage or failure in a module",
            BnrError::E_SP46 => "E_SP46: BNR has detected a blockage or failure in a module",
            BnrError::E_SP47 => "E_SP47: BNR has detected a blockage or failure in a module",
            BnrError::E_SP48 => "E_SP48: BNR has detected a blockage or failure in a module",
            BnrError::E_SP49 => "E_SP49: BNR has detected a blockage or failure in a module",
            BnrError::E_SP50 => "E_SP50: BNR has detected a blockage or failure in a module",
            BnrError::E_SP51 => "E_SP51: BNR has detected a blockage or failure in a module",
            BnrError::W_LO101 => "W_LO101: BNR is not correctly configured",
            BnrError::E_LO101 => "E_LO101: BNR cannot find the Loader module",
            BnrError::E_LO102 => "E_LO102: BNR is not correctly configured",
            BnrError::E_LO103 => "E_LO103: BNR cannot find the Loader module",
            BnrError::E_LO104 => "E_LO104: BNR is not correctly configured",
            BnrError::E_LO105 => "E_LO105: BNR is not correctly configured",
            BnrError::W_LO102 => "W_LO102: Loader module near empty",
            BnrError::W_LO103 => "W_LO103: Loader module empty",
            BnrError::E_LO106 => "E_LO106: A bill has stopped before reaching its destination",
            BnrError::E_LO107 => "E_LO107: A bill has stopped before reaching its destination",
            BnrError::E_LO108 => "E_LO108: A bill has stopped before reaching its destination",
            BnrError::E_LO109 => "E_LO109: A bill has stopped before reaching its destination",
            BnrError::E_LO110 => "E_LO110: Loader module failure",
            BnrError::E_LO111 => "E_LO111: Loader module failure",
            BnrError::E_LO112 => "E_LO112: Loader module failure",
            BnrError::E_LO113 => "E_LO113: Loader module failure",
            BnrError::E_LO114 => "E_LO114: Loader module failure",
            BnrError::E_LO115 => "E_LO115: Loader module failure",
            BnrError::E_LO116 => "E_LO116: Loader module failure",
            BnrError::E_LO117 => "E_LO117: Loader CashType does not match CashUnit configuration",
            BnrError::E_LO118 => "E_LO118: Bills extracted from Loader do not correspond to configured Loader CashType",
            BnrError::E_LO119 => "E_LO119: Bills extracted from Loader are not recognized by the BNR",
            BnrError::W_LO301 => "W_LO301: BNR is not correctly configured",
            BnrError::E_LO301 => "E_LO301: BNR cannot find the Loader module",
            BnrError::E_LO302 => "E_LO302: BNR is not correctly configured",
            BnrError::E_LO303 => "E_LO303: BNR cannot find the Loader module",
            BnrError::E_LO304 => "E_LO304: BNR is not correctly configured",
            BnrError::E_LO305 => "E_LO305: BNR is not correctly configured",
            BnrError::W_LO302 => "W_LO302: Loader module near empty",
            BnrError::W_LO303 => "W_LO303: Loader module empty",
            BnrError::E_LO306 => "E_LO306: A bill has stopped before reaching its destination",
            BnrError::E_LO307 => "E_LO307: A bill has stopped before reaching its destination",
            BnrError::E_LO308 => "E_LO308: A bill has stopped before reaching its destination",
            BnrError::E_LO309 => "E_LO309: A bill has stopped before reaching its destination",
            BnrError::E_LO310 => "E_LO310: Loader module failure",
            BnrError::E_LO311 => "E_LO311: Loader module failure",
            BnrError::E_LO312 => "E_LO312: Loader module failure",
            BnrError::E_LO313 => "E_LO313: Loader module failure",
            BnrError::E_LO314 => "E_LO314: Loader module failure",
            BnrError::E_LO315 => "E_LO315: Loader module failure",
            BnrError::E_LO316 => "E_LO316: Loader module failure",
            BnrError::E_LO317 => "E_LO317: Loader CashType does not match CashUnit configuration",
            BnrError::E_LO318 => "E_LO318: Bills extracted from Loader do not correspond to configured Loader CashType",
            BnrError::E_LO319 => "E_LO319: Bills extracted from Loader are not recognized by the BNR",
            BnrError::W_LO501 => "W_LO501: BNR is not correctly configured",
            BnrError::E_LO501 => "E_LO501: BNR cannot find the Loader module",
            BnrError::E_LO502 => "E_LO502: BNR is not correctly configured",
            BnrError::E_LO503 => "E_LO503: BNR cannot find the Loader module",
            BnrError::E_LO504 => "E_LO504: BNR is not correctly configured",
            BnrError::E_LO505 => "E_LO505: BNR is not correctly configured",
            BnrError::W_LO502 => "W_LO502: Loader module near empty",
            BnrError::W_LO503 => "W_LO503: Loader module empty",
            BnrError::E_LO506 => "E_LO506: A bill has stopped before reaching its destination",
            BnrError::E_LO507 => "E_LO507: A bill has stopped before reaching its destination",
            BnrError::E_LO508 => "E_LO508: A bill has stopped before reaching its destination",
            BnrError::E_LO509 => "E_LO509: A bill has stopped before reaching its destination",
            BnrError::E_LO510 => "E_LO510: Loader module failure",
            BnrError::E_LO511 => "E_LO511: Loader module failure",
            BnrError::E_LO512 => "E_LO512: Loader module failure",
            BnrError::E_LO513 => "E_LO513: Loader module failure",
            BnrError::E_LO514 => "E_LO514: Loader module failure",
            BnrError::E_LO515 => "E_LO515: Loader module failure",
            BnrError::E_LO516 => "E_LO516: Loader module failure",
            BnrError::E_LO517 => "E_LO517: Loader CashType does not match CashUnit configuration",
            BnrError::E_LO518 => "E_LO518: Bills extracted from Loader do not correspond to configured Loader CashType",
            BnrError::E_LO519 => "E_LO519: Bills extracted from Loader are not recognized by the BNR",
            BnrError::W_RE101 => "W_RE101: BNR is not correctly configured",
            BnrError::E_RE101 => "E_RE101: BNR cannot find a Recycler module",
            BnrError::E_RE102 => "E_RE102: BNR is not correctly configured",
            BnrError::E_RE103 => "E_RE103: BNR cannot find a Recycler module",
            BnrError::E_RE104 => "E_RE104: BNR is not correctly configured",
            BnrError::E_RE105 => "E_RE105: BNR is not correctly configured",
            BnrError::W_RE102 => "W_RE102: A Recycler module is near empty",
            BnrError::W_RE103 => "W_RE103: A Recycler module is empty",
            BnrError::W_RE104 => "W_RE104",
            BnrError::W_RE105 => "W_RE105",
            BnrError::E_RE106 => "E_RE106: BNR has detected a blockage or failure in a module",
            BnrError::E_RE107 => "E_RE107: BNR has detected a blockage or failure in a module",
            BnrError::E_RE108 => "E_RE108: BNR has detected a blockage or failure in a module",
            BnrError::E_RE109 => "E_RE109: BNR has detected a blockage or failure in a module",
            BnrError::E_RE110 => "E_RE110: BNR has detected a blockage or failure in a module",
            BnrError::E_RE111 => "E_RE111: BNR has detected a blockage or failure in a module",
            BnrError::E_RE112 => "E_RE112: BNR has detected a blockage or failure in a module",
            BnrError::E_RE113 => "E_RE113: BNR has detected a blockage or failure in a module",
            BnrError::E_RE114 => "E_RE114: BNR has detected a blockage or failure in a module",
            BnrError::E_RE115 => "E_RE115: BNR has detected a blockage or failure in a module",
            BnrError::E_RE116 => "E_RE116: Recycler CashType does not match CashUnit configuration",
            BnrError::W_RE201 => "W_RE201: BNR is not correctly configured",
            BnrError::E_RE201 => "E_RE201: BNR cannot find a Recycler module",
            BnrError::E_RE202 => "E_RE202: BNR is not correctly configured",
            BnrError::E_RE203 => "E_RE203: BNR cannot find a Recycler module",
            BnrError::E_RE204 => "E_RE204: BNR is not correctly configured",
            BnrError::E_RE205 => "E_RE205: BNR is not correctly configured",
            BnrError::W_RE202 => "W_RE202: A Recycler module is near empty",
            BnrError::W_RE203 => "W_RE203: A Recycler module is empty",
            BnrError::W_RE204 => "W_RE204",
            BnrError::W_RE205 => "W_RE205",
            BnrError::E_RE206 => "E_RE206: BNR has detected a blockage or failure in a module",
            BnrError::E_RE207 => "E_RE207: BNR has detected a blockage or failure in a module",
            BnrError::E_RE208 => "E_RE208: BNR has detected a blockage or failure in a module",
            BnrError::E_RE209 => "E_RE209: BNR has detected a blockage or failure in a module",
            BnrError::E_RE210 => "E_RE210: BNR has detected a blockage or failure in a module",
            BnrError::E_RE211 => "E_RE211: BNR has detected a blockage or failure in a module",
            BnrError::E_RE212 => "E_RE212: BNR has detected a blockage or failure in a module",
            BnrError::E_RE213 => "E_RE213: BNR has detected a blockage or failure in a module",
            BnrError::E_RE214 => "E_RE214: BNR has detected a blockage or failure in a module",
            BnrError::E_RE215 => "E_RE215: BNR has detected a blockage or failure in a module",
            BnrError::E_RE216 => "E_RE216: Recycler CashType does not match CashUnit configuration",
            BnrError::W_RE301 => "W_RE301: BNR is not correctly configured",
            BnrError::E_RE301 => "E_RE301: BNR cannot find a Recycler module",
            BnrError::E_RE302 => "E_RE302: BNR is not correctly configured",
            BnrError::E_RE303 => "E_RE303: BNR cannot find a Recycler module",
            BnrError::E_RE304 => "E_RE304: BNR is not correctly configured",
            BnrError::E_RE305 => "E_RE305: BNR is not correctly configured",
            BnrError::W_RE302 => "W_RE302: A Recycler module is near empty",
            BnrError::W_RE303 => "W_RE303: A Recycler module is empty",
            BnrError::W_RE304 => "W_RE304",
            BnrError::W_RE305 => "W_RE305",
            BnrError::E_RE306 => "E_RE306: BNR has detected a blockage or failure in a module",
            BnrError::E_RE307 => "E_RE307: BNR has detected a blockage or failure in a module",
            BnrError::E_RE308 => "E_RE308: BNR has detected a blockage or failure in a module",
            BnrError::E_RE309 => "E_RE309: BNR has detected a blockage or failure in a module",
            BnrError::E_RE310 => "E_RE310: BNR has detected a blockage or failure in a module",
            BnrError::E_RE311 => "E_RE311: BNR has detected a blockage or failure in a module",
            BnrError::E_RE312 => "E_RE312: BNR has detected a blockage or failure in a module",
            BnrError::E_RE313 => "E_RE313: BNR has detected a blockage or failure in a module",
            BnrError::E_RE314 => "E_RE314: BNR has detected a blockage or failure in a module",
            BnrError::E_RE315 => "E_RE315: BNR has detected a blockage or failure in a module",
            BnrError::E_RE316 => "E_RE316: Recycler CashType does not match CashUnit configuration",
            BnrError::W_RE401 => "W_RE401: BNR is not correctly configured",
            BnrError::E_RE401 => "E_RE401: BNR cannot find a Recycler module",
            BnrError::E_RE402 => "E_RE402: BNR is not correctly configured",
            BnrError::E_RE403 => "E_RE403: BNR cannot find a Recycler module",
            BnrError::E_RE404 => "E_RE404: BNR is not correctly configured",
            BnrError::E_RE405 => "E_RE405: BNR is not correctly configured",
            BnrError::W_RE402 => "W_RE402: A Recycler module is near empty",
            BnrError::W_RE403 => "W_RE403: A Recycler module is empty",
            BnrError::W_RE404 => "W_RE404",
            BnrError::W_RE405 => "W_RE405",
            BnrError::E_RE406 => "E_RE406: BNR has detected a blockage or failure in a module",
            BnrError::E_RE407 => "E_RE407: BNR has detected a blockage or failure in a module",
            BnrError::E_RE408 => "E_RE408: BNR has detected a blockage or failure in a module",
            BnrError::E_RE409 => "E_RE409: BNR has detected a blockage or failure in a module",
            BnrError::E_RE410 => "E_RE410: BNR has detected a blockage or failure in a module",
            BnrError::E_RE411 => "E_RE411: BNR has detected a blockage or failure in a module",
            BnrError::E_RE412 => "E_RE412: BNR has detected a blockage or failure in a module",
            BnrError::E_RE413 => "E_RE413: BNR has detected a blockage or failure in a module",
            BnrError::E_RE414 => "E_RE414: BNR has detected a blockage or failure in a module",
            BnrError::E_RE415 => "E_RE415: BNR has detected a blockage or failure in a module",
            BnrError::E_RE416 => "E_RE416: Recycler CashType does not match CashUnit configuration",
            BnrError::W_RE501 => "W_RE501: BNR is not correctly configured",
            BnrError::E_RE501 => "E_RE501: BNR cannot find a Recycler module",
            BnrError::E_RE502 => "E_RE502: BNR is not correctly configured",
            BnrError::E_RE503 => "E_RE503: BNR cannot find a Recycler module",
            BnrError::E_RE504 => "E_RE504: BNR is not correctly configured",
            BnrError::E_RE505 => "E_RE505: BNR is not correctly configured",
            BnrError::W_RE502 => "W_RE502: A Recycler module is near empty",
            BnrError::W_RE503 => "W_RE503: A Recycler module is empty",
            BnrError::W_RE504 => "W_RE504",
            BnrError::W_RE505 => "W_RE505",
            BnrError::E_RE506 => "E_RE506: BNR has detected a blockage or failure in a module",
            BnrError::E_RE507 => "E_RE507: BNR has detected a blockage or failure in a module",
            BnrError::E_RE508 => "E_RE508: BNR has detected a blockage or failure in a module",
            BnrError::E_RE509 => "E_RE509: BNR has detected a blockage or failure in a module",
            BnrError::E_RE510 => "E_RE510: BNR has detected a blockage or failure in a module",
            BnrError::E_RE511 => "E_RE511: BNR has detected a blockage or failure in a module",
            BnrError::E_RE512 => "E_RE512: BNR has detected a blockage or failure in a module",
            BnrError::E_RE513 => "E_RE513: BNR has detected a blockage or failure in a module",
            BnrError::E_RE514 => "E_RE514: BNR has detected a blockage or failure in a module",
            BnrError::E_RE515 => "E_RE515: BNR has detected a blockage or failure in a module",
            BnrError::E_RE516 => "E_RE516: Recycler CashType does not match CashUnit configuration",
            BnrError::W_RE601 => "W_RE601: BNR is not correctly configured",
            BnrError::E_RE601 => "E_RE601: BNR cannot find a Recycler module",
            BnrError::E_RE602 => "E_RE602: BNR is not correctly configured",
            BnrError::E_RE603 => "E_RE603: BNR cannot find a Recycler module",
            BnrError::E_RE604 => "E_RE604: BNR is not correctly configured",
            BnrError::E_RE605 => "E_RE605: BNR is not correctly configured",
            BnrError::W_RE602 => "W_RE602: A Recycler module is near empty",
            BnrError::W_RE603 => "W_RE603: A Recycler module is empty",
            BnrError::W_RE604 => "W_RE604",
            BnrError::W_RE605 => "W_RE605",
            BnrError::E_RE606 => "E_RE606: BNR has detected a blockage or failure in a module",
            BnrError::E_RE607 => "E_RE607: BNR has detected a blockage or failure in a module",
            BnrError::E_RE608 => "E_RE608: BNR has detected a blockage or failure in a module",
            BnrError::E_RE609 => "E_RE609: BNR has detected a blockage or failure in a module",
            BnrError::E_RE610 => "E_RE610: BNR has detected a blockage or failure in a module",
            BnrError::E_RE611 => "E_RE611: BNR has detected a blockage or failure in a module",
            BnrError::E_RE612 => "E_RE612: BNR has detected a blockage or failure in a module",
            BnrError::E_RE613 => "E_RE613: BNR has detected a blockage or failure in a module",
            BnrError::E_RE614 => "E_RE614: BNR has detected a blockage or failure in a module",
            BnrError::E_RE615 => "E_RE615: BNR has detected a blockage or failure in a module",
            BnrError::E_RE616 => "E_RE616: Recycler CashType does not match CashUnit configuration",
            BnrError::E_CB01 => "E_CB01: BNR cannot find a Cashbox module",
            BnrError::E_CB02 => "E_CB02: BNR is not correctly configured",
            BnrError::E_CB03 => "E_CB03: BNR cannot find a Cashbox module",
            BnrError::E_CB04 => "E_CB04: BNR is not correctly configured",
            BnrError::E_CB05 => "E_CB05: BNR is not correctly configured",
            BnrError::E_CB06 => "E_CB06: BNR Interlock is open",
            BnrError::E_CB07 => "E_CB07: Cashbox is not armed",
            BnrError::E_CB08 => "E_CB08: BNR is not correctly configured",
            BnrError::W_CB01 => "W_CB01: Cashbox is almost full",
            BnrError::E_CB09 => "E_CB09: Cashbox is full",
        }
    }
}

impl From<&BnrError> for &'static str {
    fn from(err: &BnrError) -> Self {
        (*err).into()
    }
}

impl std::fmt::Display for BnrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <&str>::from(self))
    }
}
