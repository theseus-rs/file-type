use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111355029: FileFormat = FileFormat {
    id: 111_355_029,
    source_type: SourceType::Wikidata,
    name: "Unreal Tournament audio package",
    extensions: &["uax"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
