use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_2347127: FileFormat = FileFormat {
    id: 2_347_127,
    source_type: SourceType::Wikidata,
    name: "Compressed image format",
    extensions: &["cso"],
    media_types: &["application/x-compressed-iso"],
    signatures: &[],
    related_formats: &[],
};
