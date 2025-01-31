use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113482236: FileFormat = FileFormat {
    id: 113_482_236,
    puid: "wikidata/113482236",
    name: "602 Graph/Chart File 1.51",
    extensions: &["gc6"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
