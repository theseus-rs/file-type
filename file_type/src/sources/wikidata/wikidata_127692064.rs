use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127692064: FileFormat = FileFormat {
    id: 127_692_064,
    source_type: SourceType::Wikidata,
    name: "freefem format",
    extensions: &["msh"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
