use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28600712: FileFormat = FileFormat {
    id: 28_600_712,
    source_type: SourceType::Wikidata,
    name: "DoItAgain",
    extensions: &["dia"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
