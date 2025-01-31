use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117844080: FileFormat = FileFormat {
    id: 117_844_080,
    puid: "wikidata/117844080",
    name: "JetFax file",
    extensions: &["jet"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
