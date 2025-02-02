use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130633933: FileFormat = FileFormat {
    id: 130_633_933,
    source_type: SourceType::Wikidata,
    name: "Ride source code file",
    extensions: &["ride"],
    media_types: &["text/x-ride"],
    internal_signatures: &[],
    related_formats: &[],
};
