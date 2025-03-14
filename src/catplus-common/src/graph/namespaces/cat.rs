use lazy_static::lazy_static;
use sophia::api::ns::Namespace;
use sophia_api::namespace;
namespace! {
    "http://example.org/cat#",
    AddAction,
    AutosamplerInjectionVolumeSetting,
    Batch,
    Campaign,
    campaignClass,
    campaignType,
    casNumber,
    chemicalName,
    ChromatogramDataCube,
    ChromatographyColumnDocument,
    containerBarcode,
    containerID,
    ContainerPositionAndQuantity,
    criteria,
    CubeStructure,
    Dataframe,
    dimension,
    Dimension,
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
    hasLiquidChromatography,
    hasObjective,
    hasSample,
    InjectionDocument,
    internalBarCode,
    measure,
    measuredQuantity,
    Measurement,
    Objective,
    Observation,
    optimizationType,
    Peak,
    PeakList,
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
    ThreeDimensionalMassSpectrumDataCube,
    ThreeDimensionalUltravioletSpectrumDataCube,
    vialShape
}
lazy_static! {
    pub static ref ns: Namespace<&'static str> = Namespace::new(PREFIX.as_str()).unwrap();
}
