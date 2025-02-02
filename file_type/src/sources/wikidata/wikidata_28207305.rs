use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207305: FileFormat = FileFormat {
    id: 28_207_305,
    source_type: SourceType::Wikidata,
    name: "True Colour Picture",
    extensions: &["trp", "tru"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
