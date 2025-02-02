use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_109916360: FileFormat = FileFormat {
    id: 109_916_360,
    source_type: SourceType::Wikidata,
    name: "JMP Data Table",
    extensions: &["jmp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
