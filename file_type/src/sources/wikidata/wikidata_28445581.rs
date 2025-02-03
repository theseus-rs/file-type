use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28445581: FileFormat = FileFormat {
    id: 28_445_581,
    source_type: SourceType::Wikidata,
    name: "ADRIFT",
    extensions: &["taf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
