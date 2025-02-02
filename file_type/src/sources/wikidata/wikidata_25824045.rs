use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_25824045: FileFormat = FileFormat {
    id: 25_824_045,
    source_type: SourceType::Wikidata,
    name: "OSM Note File",
    extensions: &["osn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
