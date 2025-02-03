use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131471298: FileFormat = FileFormat {
    id: 131_471_298,
    source_type: SourceType::Wikidata,
    name: "MGH file format",
    extensions: &["mgh"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
