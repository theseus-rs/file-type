use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27895063: FileFormat = FileFormat {
    id: 27_895_063,
    puid: "wikidata/27895063",
    name: "Windows Media Video",
    extensions: &["wm", "wmv"],
    media_types: &["video/x-ms-wmv", "video/x-ms-wmv"],
    internal_signatures: &[],
    related_formats: &[],
};
