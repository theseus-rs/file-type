use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113674284: FileFormat = FileFormat {
    id: 113_674_284,
    source_type: SourceType::Wikidata,
    name: "Final Draft Secure Copy",
    extensions: &["fds"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
