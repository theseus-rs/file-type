use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29650512: FileFormat = FileFormat {
    id: 29_650_512,
    puid: "wikidata/29650512",
    name: "packJPG",
    extensions: &["pjg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
