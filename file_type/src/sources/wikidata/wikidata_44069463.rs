use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_44069463: FileFormat = FileFormat {
    id: 44_069_463,
    source_type: SourceType::Wikidata,
    name: "STATA Data File Format, version 105",
    extensions: &["dta"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
