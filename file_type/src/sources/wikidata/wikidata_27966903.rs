use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27966903: FileFormat = FileFormat {
    id: 27_966_903,
    source_type: SourceType::Wikidata,
    name: "KSSX",
    extensions: &["kss"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
