use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61976072: FileFormat = FileFormat {
    id: 61_976_072,
    source_type: SourceType::Wikidata,
    name: "FoxPro Report",
    extensions: &["frx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
