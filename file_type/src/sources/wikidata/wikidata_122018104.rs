use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122018104: FileFormat = FileFormat {
    id: 122_018_104,
    source_type: SourceType::Wikidata,
    name: "Stationery Brochures and More Document",
    extensions: &["dtp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
