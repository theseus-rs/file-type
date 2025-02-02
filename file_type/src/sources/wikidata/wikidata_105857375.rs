use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857375: FileFormat = FileFormat {
    id: 105_857_375,
    source_type: SourceType::Wikidata,
    name: "Cura extruder definition",
    extensions: &["json"],
    media_types: &["text/json"],
    internal_signatures: &[],
    related_formats: &[],
};
