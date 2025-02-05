use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27979377: FileFormat = FileFormat {
    id: 27_979_377,
    source_type: SourceType::Wikidata,
    name: "VobSub subtitle",
    extensions: &["sub"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
