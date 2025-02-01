use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_18653981: FileFormat = FileFormat {
    id: 18_653_981,
    puid: "wikidata/18653981",
    name: "Standard Delay Format",
    extensions: &["sdf", "sdo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
