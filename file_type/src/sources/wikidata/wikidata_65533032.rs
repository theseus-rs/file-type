use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_65533032: FileFormat = FileFormat {
    id: 65_533_032,
    source_type: SourceType::Wikidata,
    name: "Menu file format",
    extensions: &["mnu"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
