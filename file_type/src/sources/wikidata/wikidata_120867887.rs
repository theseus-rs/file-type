use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_120867887: FileFormat = FileFormat {
    id: 120_867_887,
    source_type: SourceType::Wikidata,
    name: "Cumulus Category Exchange File",
    extensions: &["cce"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
