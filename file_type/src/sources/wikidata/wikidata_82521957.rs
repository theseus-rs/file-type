use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_82521957: FileFormat = FileFormat {
    id: 82_521_957,
    source_type: SourceType::Wikidata,
    name: "Portable Voice format",
    extensions: &["pvf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
