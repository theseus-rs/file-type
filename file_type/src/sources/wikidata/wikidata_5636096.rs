use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_5636096: FileFormat = FileFormat {
    id: 5_636_096,
    source_type: SourceType::Wikidata,
    name: "HTML Components",
    extensions: &["htc"],
    media_types: &["text/x-component"],
    internal_signatures: &[],
    related_formats: &[],
};
