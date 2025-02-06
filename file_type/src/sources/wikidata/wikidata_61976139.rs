use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61976139: FileFormat = FileFormat {
    id: 61_976_139,
    source_type: SourceType::Wikidata,
    name: "Microsoft Visual FoxPro Report",
    extensions: &["frx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
