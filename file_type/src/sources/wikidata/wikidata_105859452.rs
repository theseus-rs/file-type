use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859452: FileFormat = FileFormat {
    id: 105_859_452,
    puid: "wikidata/105859452",
    name: "Qt Resource Collection",
    extensions: &["qrc"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
