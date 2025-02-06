use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125298468: FileFormat = FileFormat {
    id: 125_298_468,
    source_type: SourceType::Wikidata,
    name: "Scribe",
    extensions: &["scr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
