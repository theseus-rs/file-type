use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_12034427: FileFormat = FileFormat {
    id: 12_034_427,
    source_type: SourceType::Wikidata,
    name: "LuraDocument Format",
    extensions: &["ldf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
