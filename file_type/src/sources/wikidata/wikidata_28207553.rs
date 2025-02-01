use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207553: FileFormat = FileFormat {
    id: 28_207_553,
    puid: "wikidata/28207553",
    name: "XGA",
    extensions: &["xga"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
