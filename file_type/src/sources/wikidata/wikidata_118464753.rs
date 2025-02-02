use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_118464753: FileFormat = FileFormat {
    id: 118_464_753,
    source_type: SourceType::Wikidata,
    name: "Open Media Framework Interchange 2.0",
    extensions: &["omf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
