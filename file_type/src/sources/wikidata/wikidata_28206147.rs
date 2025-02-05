use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206147: FileFormat = FileFormat {
    id: 28_206_147,
    source_type: SourceType::Wikidata,
    name: "Freedom of Press Bitamp",
    extensions: &["1", "fop"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
