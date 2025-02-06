use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859452: FileFormat = FileFormat {
    id: 105_859_452,
    source_type: SourceType::Wikidata,
    name: "Qt Resource Collection",
    extensions: &["qrc"],
    media_types: &["text/xml"],
    signatures: &[],
    related_formats: &[],
};
