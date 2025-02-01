use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51751573: FileFormat = FileFormat {
    id: 51_751_573,
    puid: "wikidata/51751573",
    name: "AutoCAD Script",
    extensions: &["scr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
