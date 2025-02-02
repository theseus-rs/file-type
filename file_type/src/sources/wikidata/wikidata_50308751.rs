use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_50308751: FileFormat = FileFormat {
    id: 50_308_751,
    source_type: SourceType::Wikidata,
    name: "MIME Email format",
    extensions: &["eml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
