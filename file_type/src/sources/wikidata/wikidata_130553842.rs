use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130553842: FileFormat = FileFormat {
    id: 130_553_842,
    source_type: SourceType::Wikidata,
    name: "QVT Operational Mapping language file format",
    extensions: &["qvto"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
