use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_128917606: FileFormat = FileFormat {
    id: 128_917_606,
    source_type: SourceType::Wikidata,
    name: "Earl Grey source code file",
    extensions: &["eg"],
    media_types: &["text/x-earl-grey"],
    signatures: &[],
    related_formats: &[],
};
