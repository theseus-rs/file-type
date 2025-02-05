use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117844080: FileFormat = FileFormat {
    id: 117_844_080,
    source_type: SourceType::Wikidata,
    name: "JetFax file",
    extensions: &["jet"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
