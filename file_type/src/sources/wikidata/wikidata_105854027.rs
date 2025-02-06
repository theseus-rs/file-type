use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854027: FileFormat = FileFormat {
    id: 105_854_027,
    source_type: SourceType::Wikidata,
    name: "Vue D'Esprit 4 Atmosphere Preset",
    extensions: &["atm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
