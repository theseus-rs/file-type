use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113495219: FileFormat = FileFormat {
    id: 113_495_219,
    source_type: SourceType::Wikidata,
    name: "CATIA Model File 3",
    extensions: &["model"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
