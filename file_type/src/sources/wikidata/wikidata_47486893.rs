use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47486893: FileFormat = FileFormat {
    id: 47_486_893,
    puid: "wikidata/47486893",
    name: "Statistical Analysis System Catalog (Unix)",
    extensions: &["sas7cat", "sc7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
