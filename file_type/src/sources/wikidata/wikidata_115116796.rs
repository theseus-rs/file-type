use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_115116796: FileFormat = FileFormat {
    id: 115_116_796,
    source_type: SourceType::Wikidata,
    name: "Gunpaint Image File",
    extensions: &["gun"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
