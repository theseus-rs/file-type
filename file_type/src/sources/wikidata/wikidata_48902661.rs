use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_48902661: FileFormat = FileFormat {
    id: 48_902_661,
    puid: "wikidata/48902661",
    name: "Harvard Graphics Presentation Slideshow",
    extensions: &["shw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
