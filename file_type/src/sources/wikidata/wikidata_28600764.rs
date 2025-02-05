use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28600764: FileFormat = FileFormat {
    id: 28_600_764,
    source_type: SourceType::Wikidata,
    name: "Electronic Arts MOV",
    extensions: &["mov"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
