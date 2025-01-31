use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51753252: FileFormat = FileFormat {
    id: 51_753_252,
    puid: "wikidata/51753252",
    name: "AutoCAD Compiled Shape/Font File",
    extensions: &["shx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
