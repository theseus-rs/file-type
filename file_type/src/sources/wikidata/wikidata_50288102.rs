use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_50288102: FileFormat = FileFormat {
    id: 50_288_102,
    puid: "wikidata/50288102",
    name: "Mathcad Document, binary file format",
    extensions: &["mcd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
