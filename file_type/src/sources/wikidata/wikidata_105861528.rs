use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861528: FileFormat = FileFormat {
    id: 105_861_528,
    source_type: SourceType::Wikidata,
    name: "Lighthouse Project",
    extensions: &["lighthouse-project"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
