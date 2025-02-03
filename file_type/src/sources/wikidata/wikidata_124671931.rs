use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_124671931: FileFormat = FileFormat {
    id: 124_671_931,
    source_type: SourceType::Wikidata,
    name: "Timed Text Markup Language Version 2",
    extensions: &["dfxp", "ttml", "xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
