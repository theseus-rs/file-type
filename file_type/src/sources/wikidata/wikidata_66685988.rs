use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66685988: FileFormat = FileFormat {
    id: 66_685_988,
    source_type: SourceType::Wikidata,
    name: "OR5",
    extensions: &["or5"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
