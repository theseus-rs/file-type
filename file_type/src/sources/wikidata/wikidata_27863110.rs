use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27863110: FileFormat = FileFormat {
    id: 27_863_110,
    puid: "wikidata/27863110",
    name: "AutoCAD Drawing, version 1.3",
    extensions: &["dwg"],
    media_types: &["image/vnd.dwg"],
    internal_signatures: &[],
    related_formats: &[],
};
