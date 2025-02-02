use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1662484: FileFormat = FileFormat {
    id: 1_662_484,
    source_type: SourceType::Wikidata,
    name: "Information Presentation Facility",
    extensions: &["inf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
