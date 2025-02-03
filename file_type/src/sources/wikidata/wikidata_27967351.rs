use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967351: FileFormat = FileFormat {
    id: 27_967_351,
    source_type: SourceType::Wikidata,
    name: "iTunes Music Library, XML variant",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
