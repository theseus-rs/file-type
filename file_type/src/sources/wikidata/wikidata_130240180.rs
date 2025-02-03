use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130240180: FileFormat = FileFormat {
    id: 130_240_180,
    source_type: SourceType::Wikidata,
    name: "Liquid template file",
    extensions: &["liquid"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
