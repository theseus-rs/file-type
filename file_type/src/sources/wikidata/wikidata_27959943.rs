use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27959943: FileFormat = FileFormat {
    id: 27_959_943,
    puid: "wikidata/27959943",
    name: "La Lossless Audio",
    extensions: &["la"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
