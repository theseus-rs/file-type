use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27960038: FileFormat = FileFormat {
    id: 27_960_038,
    puid: "wikidata/27960038",
    name: "Windows Media Audio Lossless",
    extensions: &["wma", "wmal"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
