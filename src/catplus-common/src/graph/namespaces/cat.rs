use lazy_static::lazy_static;
use sophia::api::ns::Namespace;
use sophia_api::namespace;
namespace! {
    "http://example.org/cat#",
    AddAction,
    Batch,
    Campaign,
    campaignClass,
    campaignType,
    casNumber,
    chemicalName,
    containerBarcode,
    containerID,
    ContainerPositionAndQuantity,
    criteria,
    dispenseType,
    errorMargin,
    expectedDatum,
    Experiment,
    FiltrateAction,
    genericObjective,
    hasBatch,
    hasCampaign,
    hasChemical,
    hasContainerPositionAndQuantity,
    hasObjective,
    hasSample,
    internalBarCode,
    measuredQuantity,
    Objective,
    Observation,
    optimizationType,
    reactionSubType,
    reactionType,
    role,
    Sample,
    SetPressureAction,
    SetTemperatureAction,
    SetVacuumAction,
    ShakeAction,
    speedInRPM,
    speedTumbleStirrerShape,
    subEquipmentName,
    swissCatNumber,
    temperatureShakerShape,
    temperatureTumbleStirrerShape,
    vialShape
}
lazy_static! {
    pub static ref ns: Namespace<&'static str> = Namespace::new(PREFIX.as_str()).unwrap();
}
