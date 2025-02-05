use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113841023: FileFormat = FileFormat {
    id: 113_841_023,
    source_type: SourceType::Wikidata,
    name: "JIFF",
    extensions: &["jif", "jiff"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
