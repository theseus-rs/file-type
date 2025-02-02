use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_61976072: FileFormat = FileFormat {
    id: 61_976_072,
    source_type: SourceType::Wikidata,
    name: "FoxPro Report",
    extensions: &["frx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
