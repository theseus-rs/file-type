use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28771267: FileFormat = FileFormat {
    id: 28_771_267,
    source_type: SourceType::Wikidata,
    name: "MLM",
    extensions: &["mlm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
