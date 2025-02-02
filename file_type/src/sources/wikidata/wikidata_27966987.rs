use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27966987: FileFormat = FileFormat {
    id: 27_966_987,
    source_type: SourceType::Wikidata,
    name: "AMOS Music Bank",
    extensions: &["abk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
