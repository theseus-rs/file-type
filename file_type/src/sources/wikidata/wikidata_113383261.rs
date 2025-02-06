use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113383261: FileFormat = FileFormat {
    id: 113_383_261,
    source_type: SourceType::Wikidata,
    name: "Roxio Audio Project File 11",
    extensions: &["rox"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
