use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_50288226: FileFormat = FileFormat {
    id: 50_288_226,
    puid: "wikidata/50288226",
    name: "Adobe Air, v1.5",
    extensions: &["air"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
