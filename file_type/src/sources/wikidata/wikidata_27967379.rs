use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967379: FileFormat = FileFormat {
    id: 27_967_379,
    source_type: SourceType::Wikidata,
    name: "B00",
    extensions: &["b00"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
