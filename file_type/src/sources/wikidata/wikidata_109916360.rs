use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_109916360: FileFormat = FileFormat {
    id: 109_916_360,
    source_type: SourceType::Wikidata,
    name: "JMP Data Table",
    extensions: &["jmp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
