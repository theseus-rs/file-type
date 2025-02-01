use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1384959: FileFormat = FileFormat {
    id: 1_384_959,
    puid: "wikidata/1384959",
    name: "Extensible Forms Description Language",
    extensions: &["frm", "xfd", "xfdl"],
    media_types: &[
        "application/vnd.xfdl",
        "application/vnd.xfdl",
        "application/vnd.xfdl",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
