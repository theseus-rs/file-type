use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111496844: FileFormat = FileFormat {
    id: 111_496_844,
    source_type: SourceType::Wikidata,
    name: "SPYne Containers",
    extensions: &["spy"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
