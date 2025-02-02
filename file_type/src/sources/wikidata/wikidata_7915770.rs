use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_7915770: FileFormat = FileFormat {
    id: 7_915_770,
    source_type: SourceType::Wikidata,
    name: "Variant Call Format",
    extensions: &["vcf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
