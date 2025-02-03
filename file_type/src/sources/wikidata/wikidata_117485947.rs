use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117485947: FileFormat = FileFormat {
    id: 117_485_947,
    source_type: SourceType::Wikidata,
    name: "Audacity Project File 2.x",
    extensions: &["aup"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
