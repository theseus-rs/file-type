use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_6158460: FileFormat = FileFormat {
    id: 6_158_460,
    puid: "wikidata/6158460",
    name: "Video Recording Object file",
    extensions: &["vro"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
