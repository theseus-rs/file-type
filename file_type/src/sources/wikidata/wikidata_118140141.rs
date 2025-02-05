use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_118140141: FileFormat = FileFormat {
    id: 118_140_141,
    source_type: SourceType::Wikidata,
    name: "Serenade Schematic File",
    extensions: &["sch"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
