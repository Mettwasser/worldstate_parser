use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum SyndicateType {
    ArbitersSyndicate,
    NecraloidSyndicate,
    EventSyndicate,
    CephalonSudaSyndicate,
    KahlSyndicate,
    NewLokaSyndicate,
    NightcapJournalSyndicate,
    QuillsSyndicate,
    RadioLegionSyndicate,
    RadioLegion2Syndicate,
    RadioLegion3Syndicate,
    PerrinSyndicate,
    VoxSyndicate,
    RedVeilSyndicate,
    VentKidsSyndicate,
    SteelMeridianSyndicate,
    EntratiLabSyndicate,
    HexSyndicate,
    EntratiSyndicate,
    CetusSyndicate,
    SolarisSyndicate,
    ZarimanSyndicate,

    RadioLegionIntermission(u8),
}
