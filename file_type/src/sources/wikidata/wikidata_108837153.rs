use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_108837153: FileFormat = FileFormat {
    id: 108_837_153,
    source_type: SourceType::Wikidata,
    name: "Quicken v4 Data File",
    extensions: &["qdt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
