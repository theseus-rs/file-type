use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122311153: FileFormat = FileFormat {
    id: 122_311_153,
    source_type: SourceType::Wikidata,
    name: "Open Mining Format",
    extensions: &["omf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
