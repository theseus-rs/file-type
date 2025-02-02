use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130618874: FileFormat = FileFormat {
    id: 130_618_874,
    source_type: SourceType::Wikidata,
    name: "Redcode file format",
    extensions: &["cw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
