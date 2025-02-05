use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66689327: FileFormat = FileFormat {
    id: 66_689_327,
    source_type: SourceType::Wikidata,
    name: "Access lock files",
    extensions: &["ldb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
