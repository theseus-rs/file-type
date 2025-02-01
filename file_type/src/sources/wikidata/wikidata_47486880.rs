use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47486880: FileFormat = FileFormat {
    id: 47_486_880,
    puid: "wikidata/47486880",
    name: "Statistical Analysis System Catalog (Unix)",
    extensions: &["sas7bcat", "sc7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
