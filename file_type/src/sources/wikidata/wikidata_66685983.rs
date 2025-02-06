use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66685983: FileFormat = FileFormat {
    id: 66_685_983,
    source_type: SourceType::Wikidata,
    name: "OR3",
    extensions: &["or3"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
