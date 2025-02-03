use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_126817617: FileFormat = FileFormat {
    id: 126_817_617,
    source_type: SourceType::Wikidata,
    name: "Eiffel Source Code File",
    extensions: &["e"],
    media_types: &["text/x-eiffel"],
    internal_signatures: &[],
    related_formats: &[],
};
