use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854194: FileFormat = FileFormat {
    id: 105_854_194,
    source_type: SourceType::Wikidata,
    name: "DEC-WSE Object File Format (text, start with LF)",
    extensions: &["aoff"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
