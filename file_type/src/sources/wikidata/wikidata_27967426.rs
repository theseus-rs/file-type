use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967426: FileFormat = FileFormat {
    id: 27_967_426,
    source_type: SourceType::Wikidata,
    name: "Creative Music System",
    extensions: &["cms"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
