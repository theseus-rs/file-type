use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_3959179: FileFormat = FileFormat {
    id: 3_959_179,
    source_type: SourceType::Wikidata,
    name: "shar",
    extensions: &["sha", "shar"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
