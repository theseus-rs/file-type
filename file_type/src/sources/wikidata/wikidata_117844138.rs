use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117844138: FileFormat = FileFormat {
    id: 117_844_138,
    source_type: SourceType::Wikidata,
    name: "Hayes JTFax file",
    extensions: &["jtf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
