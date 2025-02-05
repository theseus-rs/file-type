use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112652551: FileFormat = FileFormat {
    id: 112_652_551,
    source_type: SourceType::Wikidata,
    name: "Astound Actor file format",
    extensions: &["act"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
