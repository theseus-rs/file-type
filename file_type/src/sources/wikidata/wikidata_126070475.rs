use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_126070475: FileFormat = FileFormat {
    id: 126_070_475,
    source_type: SourceType::Wikidata,
    name: "Sibelius Scorch",
    extensions: &["sco"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
