use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_100426405: FileFormat = FileFormat {
    id: 100_426_405,
    puid: "wikidata/100426405",
    name: "Minitab Worksheet, version 13",
    extensions: &["mtw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
