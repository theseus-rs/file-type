use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112660808: FileFormat = FileFormat {
    id: 112_660_808,
    puid: "wikidata/112660808",
    name: "MediaView file",
    extensions: &["m14"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
