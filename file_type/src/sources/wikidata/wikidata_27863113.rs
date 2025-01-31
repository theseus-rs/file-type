use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27863113: FileFormat = FileFormat {
    id: 27_863_113,
    puid: "wikidata/27863113",
    name: "AutoCAD Drawing, version 2.0",
    extensions: &["dwg"],
    media_types: &["image/vnd.dwg"],
    internal_signatures: &[],
    related_formats: &[],
};
