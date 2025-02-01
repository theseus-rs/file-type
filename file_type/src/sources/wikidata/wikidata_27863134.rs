use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27863134: FileFormat = FileFormat {
    id: 27_863_134,
    puid: "wikidata/27863134",
    name: "AutoCAD Drawing, version 2004-2005",
    extensions: &["dwg"],
    media_types: &["image/vnd.dwg"],
    internal_signatures: &[],
    related_formats: &[],
};
