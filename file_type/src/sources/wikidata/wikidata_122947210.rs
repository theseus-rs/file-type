use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122947210: FileFormat = FileFormat {
    id: 122_947_210,
    puid: "wikidata/122947210",
    name: "Windows Enhanced Metafile, version 1.0",
    extensions: &["emf", "emz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
