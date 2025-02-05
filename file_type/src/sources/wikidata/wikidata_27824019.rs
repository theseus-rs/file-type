use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27824019: FileFormat = FileFormat {
    id: 27_824_019,
    source_type: SourceType::Wikidata,
    name: "ar, System V variant",
    extensions: &["a"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
