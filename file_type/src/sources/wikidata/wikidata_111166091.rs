use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111166091: FileFormat = FileFormat {
    id: 111_166_091,
    source_type: SourceType::Wikidata,
    name: "Ludwig song file",
    extensions: &["ludwig"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
