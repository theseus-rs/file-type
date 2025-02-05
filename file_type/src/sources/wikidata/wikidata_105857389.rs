use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857389: FileFormat = FileFormat {
    id: 105_857_389,
    source_type: SourceType::Wikidata,
    name: "JSON Entity Model",
    extensions: &["jem"],
    media_types: &["text/json"],
    signatures: &[],
    related_formats: &[],
};
