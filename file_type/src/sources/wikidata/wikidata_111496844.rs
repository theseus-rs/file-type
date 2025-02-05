use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111496844: FileFormat = FileFormat {
    id: 111_496_844,
    source_type: SourceType::Wikidata,
    name: "SPYne Containers",
    extensions: &["spy"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
