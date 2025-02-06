use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122169695: FileFormat = FileFormat {
    id: 122_169_695,
    source_type: SourceType::Wikidata,
    name: "Key Cache File",
    extensions: &["ekc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
