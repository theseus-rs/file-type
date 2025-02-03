use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858277: FileFormat = FileFormat {
    id: 105_858_277,
    source_type: SourceType::Wikidata,
    name: "Entity Data Model",
    extensions: &["edmx"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
