use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125857400: FileFormat = FileFormat {
    id: 125_857_400,
    puid: "wikidata/125857400",
    name: "JScript Encoded File",
    extensions: &["jse"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
