use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110238528: FileFormat = FileFormat {
    id: 110_238_528,
    puid: "wikidata/110238528",
    name: "Screenwriter 2000 Document",
    extensions: &["stw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
