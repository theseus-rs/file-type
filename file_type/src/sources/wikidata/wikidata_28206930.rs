use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206930: FileFormat = FileFormat {
    id: 28_206_930,
    source_type: SourceType::Wikidata,
    name: "PGX",
    extensions: &["pgx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
