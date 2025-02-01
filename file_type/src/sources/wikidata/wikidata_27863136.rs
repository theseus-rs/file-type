use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27863136: FileFormat = FileFormat {
    id: 27_863_136,
    puid: "wikidata/27863136",
    name: "AutoCAD Drawing, version 2007-2008",
    extensions: &["dwg"],
    media_types: &["image/vnd.dwg"],
    internal_signatures: &[],
    related_formats: &[],
};
