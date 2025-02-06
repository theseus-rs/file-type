use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206599: FileFormat = FileFormat {
    id: 28_206_599,
    source_type: SourceType::Wikidata,
    name: "MIX",
    extensions: &["mix"],
    media_types: &["image/vnd.mix"],
    signatures: &[],
    related_formats: &[],
};
