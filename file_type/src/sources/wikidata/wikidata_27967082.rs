use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967082: FileFormat = FileFormat {
    id: 27_967_082,
    source_type: SourceType::Wikidata,
    name: "David Whittaker",
    extensions: &["dw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
