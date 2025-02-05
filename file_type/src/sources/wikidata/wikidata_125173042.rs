use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125173042: FileFormat = FileFormat {
    id: 125_173_042,
    source_type: SourceType::Wikidata,
    name: "Tomboy note",
    extensions: &["note"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
