use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131322623: FileFormat = FileFormat {
    id: 131_322_623,
    source_type: SourceType::Wikidata,
    name: "TSX",
    extensions: &["tsx"],
    media_types: &["text/typescript-tsx"],
    signatures: &[],
    related_formats: &[],
};
