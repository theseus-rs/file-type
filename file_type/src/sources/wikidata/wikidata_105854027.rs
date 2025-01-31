use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854027: FileFormat = FileFormat {
    id: 105_854_027,
    puid: "wikidata/105854027",
    name: "Vue D'Esprit 4 Atmosphere Preset",
    extensions: &["atm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
