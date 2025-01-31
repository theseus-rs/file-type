use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_100424447: FileFormat = FileFormat {
    id: 100_424_447,
    puid: "wikidata/100424447",
    name: "Minitab Worksheet, version 8-11",
    extensions: &["mtw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
