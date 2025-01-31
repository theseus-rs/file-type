use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130642484: FileFormat = FileFormat {
    id: 130_642_484,
    puid: "wikidata/130642484",
    name: "Roboconf instances file",
    extensions: &["instances"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
