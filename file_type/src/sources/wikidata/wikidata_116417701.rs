use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116417701: FileFormat = FileFormat {
    id: 116_417_701,
    source_type: SourceType::Wikidata,
    name: "Design and Print file",
    extensions: &["bro"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
