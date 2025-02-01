use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131322623: FileFormat = FileFormat {
    id: 131_322_623,
    puid: "wikidata/131322623",
    name: "TSX",
    extensions: &["tsx"],
    media_types: &["text/typescript-tsx"],
    internal_signatures: &[],
    related_formats: &[],
};
