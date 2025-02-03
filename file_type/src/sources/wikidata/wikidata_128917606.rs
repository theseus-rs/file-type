use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_128917606: FileFormat = FileFormat {
    id: 128_917_606,
    source_type: SourceType::Wikidata,
    name: "Earl Grey source code file",
    extensions: &["eg"],
    media_types: &["text/x-earl-grey"],
    internal_signatures: &[],
    related_formats: &[],
};
