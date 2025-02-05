use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114050725: FileFormat = FileFormat {
    id: 114_050_725,
    source_type: SourceType::Wikidata,
    name: "Canon CIF File",
    extensions: &["cif"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
