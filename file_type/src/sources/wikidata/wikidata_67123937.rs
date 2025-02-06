use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_67123937: FileFormat = FileFormat {
    id: 67_123_937,
    source_type: SourceType::Wikidata,
    name: "Print Artist business card file format",
    extensions: &["bc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
