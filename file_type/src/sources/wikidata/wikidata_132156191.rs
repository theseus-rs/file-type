use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_132156191: FileFormat = FileFormat {
    id: 132_156_191,
    source_type: SourceType::Wikidata,
    name: "NIMAS zipped file",
    extensions: &["zip"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
