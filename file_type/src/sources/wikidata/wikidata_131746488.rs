use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131746488: FileFormat = FileFormat {
    id: 131_746_488,
    source_type: SourceType::Wikidata,
    name: "Natron Project File",
    extensions: &["ntp"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
