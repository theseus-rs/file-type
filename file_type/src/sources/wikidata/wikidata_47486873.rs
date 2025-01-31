use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47486873: FileFormat = FileFormat {
    id: 47_486_873,
    puid: "wikidata/47486873",
    name: "Statistical Analysis System Catalog (Windows)",
    extensions: &["sas7bcat", "sc7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
