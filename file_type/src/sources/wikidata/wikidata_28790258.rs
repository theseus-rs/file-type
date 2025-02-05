use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28790258: FileFormat = FileFormat {
    id: 28_790_258,
    source_type: SourceType::Wikidata,
    name: "maz",
    extensions: &["maz"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
