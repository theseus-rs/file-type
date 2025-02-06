use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111317689: FileFormat = FileFormat {
    id: 111_317_689,
    source_type: SourceType::Wikidata,
    name: "Miles Sound System DLS 1 + XMI file",
    extensions: &["mss"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
