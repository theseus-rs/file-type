use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111317689: FileFormat = FileFormat {
    id: 111_317_689,
    source_type: SourceType::Wikidata,
    name: "Miles Sound System DLS 1 + XMI file",
    extensions: &["mss"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
