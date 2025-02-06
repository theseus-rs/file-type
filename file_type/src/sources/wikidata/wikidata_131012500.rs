use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131012500: FileFormat = FileFormat {
    id: 131_012_500,
    source_type: SourceType::Wikidata,
    name: "Stringified NBT format",
    extensions: &["snbt"],
    media_types: &["text/snbt"],
    signatures: &[],
    related_formats: &[],
};
