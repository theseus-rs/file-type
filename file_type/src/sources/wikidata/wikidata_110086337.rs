use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110086337: FileFormat = FileFormat {
    id: 110_086_337,
    puid: "wikidata/110086337",
    name: "Cool Edit/Adobe Audition Session File (Binary)",
    extensions: &["ses"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
