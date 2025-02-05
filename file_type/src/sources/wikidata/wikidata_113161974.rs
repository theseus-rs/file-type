use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113161974: FileFormat = FileFormat {
    id: 113_161_974,
    source_type: SourceType::Wikidata,
    name: "Act! database file",
    extensions: &["dbf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
