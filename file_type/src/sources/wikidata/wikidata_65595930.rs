use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_65595930: FileFormat = FileFormat {
    id: 65_595_930,
    source_type: SourceType::Wikidata,
    name: "Variant Call Format",
    extensions: &["vcf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
