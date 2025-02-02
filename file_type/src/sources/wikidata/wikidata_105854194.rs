use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854194: FileFormat = FileFormat {
    id: 105_854_194,
    source_type: SourceType::Wikidata,
    name: "DEC-WSE Object File Format (text, start with LF)",
    extensions: &["aoff"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
