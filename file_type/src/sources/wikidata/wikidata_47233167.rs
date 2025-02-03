use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47233167: FileFormat = FileFormat {
    id: 47_233_167,
    source_type: SourceType::Wikidata,
    name: "LDR",
    extensions: &["dat", "ldr"],
    media_types: &["application/x-ldraw"],
    internal_signatures: &[],
    related_formats: &[],
};
