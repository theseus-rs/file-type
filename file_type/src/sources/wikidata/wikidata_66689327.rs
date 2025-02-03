use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_66689327: FileFormat = FileFormat {
    id: 66_689_327,
    source_type: SourceType::Wikidata,
    name: "Access lock files",
    extensions: &["ldb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
