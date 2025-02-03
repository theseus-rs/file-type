use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113556907: FileFormat = FileFormat {
    id: 113_556_907,
    source_type: SourceType::Wikidata,
    name: "Duplicator CD Image File",
    extensions: &["tao"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
