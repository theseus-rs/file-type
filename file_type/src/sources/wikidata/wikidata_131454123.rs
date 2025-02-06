use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131454123: FileFormat = FileFormat {
    id: 131_454_123,
    source_type: SourceType::Wikidata,
    name: "Zig file format",
    extensions: &["zig"],
    media_types: &["text/zig"],
    signatures: &[],
    related_formats: &[],
};
