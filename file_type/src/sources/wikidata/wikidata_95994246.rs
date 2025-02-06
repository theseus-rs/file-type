use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_95994246: FileFormat = FileFormat {
    id: 95_994_246,
    source_type: SourceType::Wikidata,
    name: "Agilent Microarray file format",
    extensions: &["txt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
