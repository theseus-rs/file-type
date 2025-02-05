use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857375: FileFormat = FileFormat {
    id: 105_857_375,
    source_type: SourceType::Wikidata,
    name: "Cura extruder definition",
    extensions: &["json"],
    media_types: &["text/json"],
    signatures: &[],
    related_formats: &[],
};
