use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122947210: FileFormat = FileFormat {
    id: 122_947_210,
    source_type: SourceType::Wikidata,
    name: "Windows Enhanced Metafile, version 1.0",
    extensions: &["emf", "emz"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
