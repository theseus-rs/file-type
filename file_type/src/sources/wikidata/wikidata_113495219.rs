use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113495219: FileFormat = FileFormat {
    id: 113_495_219,
    source_type: SourceType::Wikidata,
    name: "CATIA Model File 3",
    extensions: &["model"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
