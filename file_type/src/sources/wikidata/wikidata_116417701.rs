use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116417701: FileFormat = FileFormat {
    id: 116_417_701,
    source_type: SourceType::Wikidata,
    name: "Design and Print file",
    extensions: &["bro"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
