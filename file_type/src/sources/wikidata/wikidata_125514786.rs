use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125514786: FileFormat = FileFormat {
    id: 125_514_786,
    source_type: SourceType::Wikidata,
    name: "Hasselblad RAW Image",
    extensions: &["fff"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
