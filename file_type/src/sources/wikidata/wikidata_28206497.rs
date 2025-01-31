use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206497: FileFormat = FileFormat {
    id: 28_206_497,
    puid: "wikidata/28206497",
    name: "Lossless JPEG",
    extensions: &["jpg", "ljpeg", "ljpg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
