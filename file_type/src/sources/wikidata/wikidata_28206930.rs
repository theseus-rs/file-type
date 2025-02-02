use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206930: FileFormat = FileFormat {
    id: 28_206_930,
    source_type: SourceType::Wikidata,
    name: "PGX",
    extensions: &["pgx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
