use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113481800: FileFormat = FileFormat {
    id: 113_481_800,
    puid: "wikidata/113481800",
    name: "602 Text file 1.0-1.51",
    extensions: &["602"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
