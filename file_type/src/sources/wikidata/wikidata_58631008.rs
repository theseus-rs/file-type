use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_58631008: FileFormat = FileFormat {
    id: 58_631_008,
    source_type: SourceType::Wikidata,
    name: "Harris Matrix",
    extensions: &["hm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
