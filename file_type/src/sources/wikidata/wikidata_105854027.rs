use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854027: FileFormat = FileFormat {
    id: 105_854_027,
    source_type: SourceType::Wikidata,
    name: "Vue D'Esprit 4 Atmosphere Preset",
    extensions: &["atm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
