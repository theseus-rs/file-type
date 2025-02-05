use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111665220: FileFormat = FileFormat {
    id: 111_665_220,
    source_type: SourceType::Wikidata,
    name: "OpenDocument Text Template",
    extensions: &["ott"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
