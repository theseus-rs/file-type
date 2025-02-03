use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852177: FileFormat = FileFormat {
    id: 105_852_177,
    source_type: SourceType::Wikidata,
    name: "SuperTux Level (with rem)",
    extensions: &["stl"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
