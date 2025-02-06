use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_25824045: FileFormat = FileFormat {
    id: 25_824_045,
    source_type: SourceType::Wikidata,
    name: "OSM Note File",
    extensions: &["osn"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
