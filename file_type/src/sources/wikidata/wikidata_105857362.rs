use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857362: FileFormat = FileFormat {
    id: 105_857_362,
    source_type: SourceType::Wikidata,
    name: "QMK keymap",
    extensions: &["json"],
    media_types: &["text/json"],
    internal_signatures: &[],
    related_formats: &[],
};
