use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125207315: FileFormat = FileFormat {
    id: 125_207_315,
    source_type: SourceType::Wikidata,
    name: "VYM part",
    extensions: &["vyp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
