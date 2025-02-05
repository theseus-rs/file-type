use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_25101636: FileFormat = FileFormat {
    id: 25_101_636,
    source_type: SourceType::Wikidata,
    name: "IVUE",
    extensions: &["ivue"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
