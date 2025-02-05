use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27824032: FileFormat = FileFormat {
    id: 27_824_032,
    source_type: SourceType::Wikidata,
    name: "ar, Sixth Edition Unix variant",
    extensions: &["a"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
