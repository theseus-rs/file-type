use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852177: FileFormat = FileFormat {
    id: 105_852_177,
    source_type: SourceType::Wikidata,
    name: "SuperTux Level (with rem)",
    extensions: &["stl"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
