use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122829936: FileFormat = FileFormat {
    id: 122_829_936,
    source_type: SourceType::Wikidata,
    name: "Creativity Workshop PWK file",
    extensions: &["pwk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
