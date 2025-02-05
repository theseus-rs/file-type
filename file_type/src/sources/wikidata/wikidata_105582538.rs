use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105582538: FileFormat = FileFormat {
    id: 105_582_538,
    source_type: SourceType::Wikidata,
    name: "JavaScript modules",
    extensions: &["mjs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
