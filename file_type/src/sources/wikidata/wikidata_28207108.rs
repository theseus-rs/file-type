use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207108: FileFormat = FileFormat {
    id: 28_207_108,
    source_type: SourceType::Wikidata,
    name: "The Print Shop Graphics file",
    extensions: &["dat"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
