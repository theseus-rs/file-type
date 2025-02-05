use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122730741: FileFormat = FileFormat {
    id: 122_730_741,
    source_type: SourceType::Wikidata,
    name: "Lazer View file",
    extensions: &["lv"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
