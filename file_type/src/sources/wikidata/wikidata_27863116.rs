use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27863116: FileFormat = FileFormat {
    id: 27_863_116,
    puid: "wikidata/27863116",
    name: "AutoCAD Drawing, version 2.1",
    extensions: &["dwg"],
    media_types: &["image/vnd.dwg"],
    internal_signatures: &[],
    related_formats: &[],
};
