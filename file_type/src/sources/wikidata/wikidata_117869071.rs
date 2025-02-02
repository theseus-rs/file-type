use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117869071: FileFormat = FileFormat {
    id: 117_869_071,
    source_type: SourceType::Wikidata,
    name: "Relisys file",
    extensions: &["tef"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
