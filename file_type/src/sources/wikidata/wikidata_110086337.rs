use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110086337: FileFormat = FileFormat {
    id: 110_086_337,
    source_type: SourceType::Wikidata,
    name: "Cool Edit/Adobe Audition Session File (Binary)",
    extensions: &["ses"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
