use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967082: FileFormat = FileFormat {
    id: 27_967_082,
    source_type: SourceType::Wikidata,
    name: "David Whittaker",
    extensions: &["dw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
