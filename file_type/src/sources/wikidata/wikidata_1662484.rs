use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1662484: FileFormat = FileFormat {
    id: 1_662_484,
    source_type: SourceType::Wikidata,
    name: "Information Presentation Facility",
    extensions: &["inf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
