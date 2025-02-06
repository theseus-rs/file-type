use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110086337: FileFormat = FileFormat {
    id: 110_086_337,
    source_type: SourceType::Wikidata,
    name: "Cool Edit/Adobe Audition Session File (Binary)",
    extensions: &["ses"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
