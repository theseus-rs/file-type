use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130979811: FileFormat = FileFormat {
    id: 130_979_811,
    source_type: SourceType::Wikidata,
    name: "Slim file format",
    extensions: &["slim"],
    media_types: &["text/x-slim"],
    internal_signatures: &[],
    related_formats: &[],
};
