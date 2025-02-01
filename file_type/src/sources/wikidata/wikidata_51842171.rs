use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51842171: FileFormat = FileFormat {
    id: 51_842_171,
    puid: "wikidata/51842171",
    name: "MacPaint Graphics",
    extensions: &["pnt"],
    media_types: &["image/mac"],
    internal_signatures: &[],
    related_formats: &[],
};
