use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_86920: FileFormat = FileFormat {
    id: 86_920,
    source_type: SourceType::Wikidata,
    name: "text file",
    extensions: &["text", "txt"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
