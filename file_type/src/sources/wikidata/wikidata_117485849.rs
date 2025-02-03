use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117485849: FileFormat = FileFormat {
    id: 117_485_849,
    source_type: SourceType::Wikidata,
    name: "Audacity Project File 1.x",
    extensions: &["aup"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
