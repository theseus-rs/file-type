use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113841023: FileFormat = FileFormat {
    id: 113_841_023,
    source_type: SourceType::Wikidata,
    name: "JIFF",
    extensions: &["jif", "jiff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
