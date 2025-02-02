use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_44069463: FileFormat = FileFormat {
    id: 44_069_463,
    source_type: SourceType::Wikidata,
    name: "STATA Data File Format, version 105",
    extensions: &["dta"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
