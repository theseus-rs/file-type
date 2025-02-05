use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_25824152: FileFormat = FileFormat {
    id: 25_824_152,
    source_type: SourceType::Wikidata,
    name: "JOSM Session File",
    extensions: &["jos"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
