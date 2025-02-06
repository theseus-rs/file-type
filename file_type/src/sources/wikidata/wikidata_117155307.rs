use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117155307: FileFormat = FileFormat {
    id: 117_155_307,
    source_type: SourceType::Wikidata,
    name: "Picture It! PNG Plus",
    extensions: &["png"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
