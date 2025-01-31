use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_3027596: FileFormat = FileFormat {
    id: 3_027_596,
    puid: "wikidata/3027596",
    name: "DGN",
    extensions: &["dgn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
