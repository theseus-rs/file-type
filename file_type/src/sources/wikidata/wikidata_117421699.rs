use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117421699: FileFormat = FileFormat {
    id: 117_421_699,
    puid: "wikidata/117421699",
    name: "JSONC",
    extensions: &["jsonc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
