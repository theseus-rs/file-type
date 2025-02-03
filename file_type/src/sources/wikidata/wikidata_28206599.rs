use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206599: FileFormat = FileFormat {
    id: 28_206_599,
    source_type: SourceType::Wikidata,
    name: "MIX",
    extensions: &["mix"],
    media_types: &["image/vnd.mix"],
    internal_signatures: &[],
    related_formats: &[],
};
