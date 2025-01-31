use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967435: FileFormat = FileFormat {
    id: 27_967_435,
    puid: "wikidata/27967435",
    name: "Bink Video, version 2",
    extensions: &["bik2", "bk2"],
    media_types: &[
        "video/vnd.radgamettools.bink",
        "video/vnd.radgamettools.bink",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
