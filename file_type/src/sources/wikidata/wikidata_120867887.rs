use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_120867887: FileFormat = FileFormat {
    id: 120_867_887,
    source_type: SourceType::Wikidata,
    name: "Cumulus Category Exchange File",
    extensions: &["cce"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
