use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_118139731: FileFormat = FileFormat {
    id: 118_139_731,
    source_type: SourceType::Wikidata,
    name: "Printable Project",
    extensions: &["gwp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
