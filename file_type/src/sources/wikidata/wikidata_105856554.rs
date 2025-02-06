use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856554: FileFormat = FileFormat {
    id: 105_856_554,
    source_type: SourceType::Wikidata,
    name: "Weaving Interchange Format",
    extensions: &["wif"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
