use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125134559: FileFormat = FileFormat {
    id: 125_134_559,
    source_type: SourceType::Wikidata,
    name: "YAM Dictionary",
    extensions: &["glossary"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
