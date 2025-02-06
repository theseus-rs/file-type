use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_70081522: FileFormat = FileFormat {
    id: 70_081_522,
    source_type: SourceType::Wikidata,
    name: "TextPipe Filter List file format",
    extensions: &["fll"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
