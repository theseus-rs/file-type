use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_126070475: FileFormat = FileFormat {
    id: 126_070_475,
    source_type: SourceType::Wikidata,
    name: "Sibelius Scorch",
    extensions: &["sco"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
