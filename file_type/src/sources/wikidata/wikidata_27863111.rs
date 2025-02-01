use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27863111: FileFormat = FileFormat {
    id: 27_863_111,
    puid: "wikidata/27863111",
    name: "AutoCAD Drawing, version 1.4",
    extensions: &["dwg"],
    media_types: &["image/vnd.dwg"],
    internal_signatures: &[],
    related_formats: &[],
};
