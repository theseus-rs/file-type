use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_119444389: FileFormat = FileFormat {
    id: 119_444_389,
    puid: "wikidata/119444389",
    name: "Comic Book Template",
    extensions: &["cbtx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
