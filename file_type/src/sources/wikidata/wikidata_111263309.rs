use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111263309: FileFormat = FileFormat {
    id: 111_263_309,
    source_type: SourceType::Wikidata,
    name: "Sound Designer I file",
    extensions: &["dig", "sd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
