use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111440951: FileFormat = FileFormat {
    id: 111_440_951,
    source_type: SourceType::Wikidata,
    name: "BASIC Source Code File",
    extensions: &["bas"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
