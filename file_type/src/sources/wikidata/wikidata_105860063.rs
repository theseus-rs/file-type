use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860063: FileFormat = FileFormat {
    id: 105_860_063,
    source_type: SourceType::Wikidata,
    name: "ParaView VTK Image data",
    extensions: &["vti"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
