use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_89101317: FileFormat = FileFormat {
    id: 89_101_317,
    puid: "wikidata/89101317",
    name: "PrintMaster Gold Project 1",
    extensions: &["ban", "cal", "car", "let", "sig"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
