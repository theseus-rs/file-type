use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205790: FileFormat = FileFormat {
    id: 28_205_790,
    puid: "wikidata/28205790",
    name: "FullPic Picture Format",
    extensions: &["ful"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
