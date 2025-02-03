use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117155307: FileFormat = FileFormat {
    id: 117_155_307,
    source_type: SourceType::Wikidata,
    name: "Picture It! PNG Plus",
    extensions: &["png"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
