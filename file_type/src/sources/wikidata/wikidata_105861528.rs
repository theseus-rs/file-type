use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861528: FileFormat = FileFormat {
    id: 105_861_528,
    source_type: SourceType::Wikidata,
    name: "Lighthouse Project",
    extensions: &["lighthouse-project"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
