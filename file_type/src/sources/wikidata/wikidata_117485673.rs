use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117485673: FileFormat = FileFormat {
    id: 117_485_673,
    source_type: SourceType::Wikidata,
    name: "Audacity Project File (Early)",
    extensions: &["aup"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
