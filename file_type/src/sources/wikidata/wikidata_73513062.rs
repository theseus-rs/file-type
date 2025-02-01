use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_73513062: FileFormat = FileFormat {
    id: 73_513_062,
    puid: "wikidata/73513062",
    name: "Pathetic Writer document",
    extensions: &["pw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
