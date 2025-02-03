use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113674284: FileFormat = FileFormat {
    id: 113_674_284,
    source_type: SourceType::Wikidata,
    name: "Final Draft Secure Copy",
    extensions: &["fds"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
