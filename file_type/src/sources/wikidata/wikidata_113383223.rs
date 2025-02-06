use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113383223: FileFormat = FileFormat {
    id: 113_383_223,
    source_type: SourceType::Wikidata,
    name: "Roxio Data Project File 11",
    extensions: &["rox"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
