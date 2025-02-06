use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207000: FileFormat = FileFormat {
    id: 28_207_000,
    source_type: SourceType::Wikidata,
    name: "Picture Packer",
    extensions: &["pp1", "pp2", "pp3"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
