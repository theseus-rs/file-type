use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122947259: FileFormat = FileFormat {
    id: 122_947_259,
    puid: "wikidata/122947259",
    name: "Windows Enhanced Metafile, version 2.0",
    extensions: &["emf", "emz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
