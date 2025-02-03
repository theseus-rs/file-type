use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111355029: FileFormat = FileFormat {
    id: 111_355_029,
    source_type: SourceType::Wikidata,
    name: "Unreal Tournament audio package",
    extensions: &["uax"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
