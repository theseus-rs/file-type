use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51718111: FileFormat = FileFormat {
    id: 51_718_111,
    puid: "wikidata/51718111",
    name: "AutoCAD ACIS Export File",
    extensions: &["sat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
