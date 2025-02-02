use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207125: FileFormat = FileFormat {
    id: 28_207_125,
    source_type: SourceType::Wikidata,
    name: "The New Print Shop Graphics file",
    extensions: &["pog"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
