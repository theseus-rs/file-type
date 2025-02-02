use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116784985: FileFormat = FileFormat {
    id: 116_784_985,
    source_type: SourceType::Wikidata,
    name: "Project Manager Pro Chart file",
    extensions: &["pmp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
