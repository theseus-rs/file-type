use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130553842: FileFormat = FileFormat {
    id: 130_553_842,
    source_type: SourceType::Wikidata,
    name: "QVT Operational Mapping language file format",
    extensions: &["qvto"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
