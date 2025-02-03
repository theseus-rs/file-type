use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856554: FileFormat = FileFormat {
    id: 105_856_554,
    source_type: SourceType::Wikidata,
    name: "Weaving Interchange Format",
    extensions: &["wif"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
