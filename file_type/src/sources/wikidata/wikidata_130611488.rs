use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130611488: FileFormat = FileFormat {
    id: 130_611_488,
    source_type: SourceType::Wikidata,
    name: "Red language file format",
    extensions: &["red", "reds"],
    media_types: &["text/x-red", "text/x-red-system"],
    signatures: &[],
    related_formats: &[],
};
