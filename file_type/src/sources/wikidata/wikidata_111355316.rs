use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111355316: FileFormat = FileFormat {
    id: 111_355_316,
    puid: "wikidata/111355316",
    name: "UltraTracker wave file",
    extensions: &["uwf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
