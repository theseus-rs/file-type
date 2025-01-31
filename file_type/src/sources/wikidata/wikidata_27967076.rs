use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967076: FileFormat = FileFormat {
    id: 27_967_076,
    puid: "wikidata/27967076",
    name: "Audio Sculpture",
    extensions: &["adsc", "as"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
