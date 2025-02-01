use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122947391: FileFormat = FileFormat {
    id: 122_947_391,
    puid: "wikidata/122947391",
    name: "Windows Enhanced Metafile, version 3.0",
    extensions: &["emf", "emz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
