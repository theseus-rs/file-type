use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111332298: FileFormat = FileFormat {
    id: 111_332_298,
    puid: "wikidata/111332298",
    name: "Typhoon voice file",
    extensions: &["o01"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
