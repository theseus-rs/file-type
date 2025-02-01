use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206930: FileFormat = FileFormat {
    id: 28_206_930,
    puid: "wikidata/28206930",
    name: "PGX",
    extensions: &["pgx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
