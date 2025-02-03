use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967422: FileFormat = FileFormat {
    id: 27_967_422,
    source_type: SourceType::Wikidata,
    name: "ChordML",
    extensions: &["cml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
