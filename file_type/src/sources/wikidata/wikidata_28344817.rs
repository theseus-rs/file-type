use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28344817: FileFormat = FileFormat {
    id: 28_344_817,
    source_type: SourceType::Wikidata,
    name: "Arts and Letters clip art library",
    extensions: &["yal"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
