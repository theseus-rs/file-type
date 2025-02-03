use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207108: FileFormat = FileFormat {
    id: 28_207_108,
    source_type: SourceType::Wikidata,
    name: "The Print Shop Graphics file",
    extensions: &["dat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
