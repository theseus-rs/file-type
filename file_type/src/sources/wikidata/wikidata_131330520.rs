use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131330520: FileFormat = FileFormat {
    id: 131_330_520,
    source_type: SourceType::Wikidata,
    name: "Typographic Number Theory file format",
    extensions: &["tnt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
