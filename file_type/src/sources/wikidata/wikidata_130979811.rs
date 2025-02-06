use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130979811: FileFormat = FileFormat {
    id: 130_979_811,
    source_type: SourceType::Wikidata,
    name: "Slim file format",
    extensions: &["slim"],
    media_types: &["text/x-slim"],
    signatures: &[],
    related_formats: &[],
};
