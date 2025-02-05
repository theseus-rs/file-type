use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27966903: FileFormat = FileFormat {
    id: 27_966_903,
    source_type: SourceType::Wikidata,
    name: "KSSX",
    extensions: &["kss"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
