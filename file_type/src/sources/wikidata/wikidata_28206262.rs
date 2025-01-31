use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206262: FileFormat = FileFormat {
    id: 28_206_262,
    puid: "wikidata/28206262",
    name: "HSI JPEG",
    extensions: &["hsi", "jpg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
