use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27863143: FileFormat = FileFormat {
    id: 27_863_143,
    puid: "wikidata/27863143",
    name: "AutoCAD Drawing, version 2013-2014",
    extensions: &["dwg"],
    media_types: &["image/vnd.dwg"],
    internal_signatures: &[],
    related_formats: &[],
};
