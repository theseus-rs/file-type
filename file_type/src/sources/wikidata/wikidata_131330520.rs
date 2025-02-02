use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131330520: FileFormat = FileFormat {
    id: 131_330_520,
    source_type: SourceType::Wikidata,
    name: "Typographic Number Theory file format",
    extensions: &["tnt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
