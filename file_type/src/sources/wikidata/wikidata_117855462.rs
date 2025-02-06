use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117855462: FileFormat = FileFormat {
    id: 117_855_462,
    source_type: SourceType::Wikidata,
    name: "PRODUCT R&D Fax File",
    extensions: &["prd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
