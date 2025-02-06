use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207305: FileFormat = FileFormat {
    id: 28_207_305,
    source_type: SourceType::Wikidata,
    name: "True Colour Picture",
    extensions: &["trp", "tru"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
