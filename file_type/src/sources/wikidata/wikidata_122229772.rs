use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122229772: FileFormat = FileFormat {
    id: 122_229_772,
    source_type: SourceType::Wikidata,
    name: "Digital Interface Format",
    extensions: &["dif"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
