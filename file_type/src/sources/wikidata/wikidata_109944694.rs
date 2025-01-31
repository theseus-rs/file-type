use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_109944694: FileFormat = FileFormat {
    id: 109_944_694,
    puid: "wikidata/109944694",
    name: "Archives file format",
    extensions: &["arv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
