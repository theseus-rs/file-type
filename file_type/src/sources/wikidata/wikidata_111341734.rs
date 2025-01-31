use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111341734: FileFormat = FileFormat {
    id: 111_341_734,
    puid: "wikidata/111341734",
    name: "Sound Designer II data forks",
    extensions: &["sd2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
