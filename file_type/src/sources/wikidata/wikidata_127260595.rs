use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127260595: FileFormat = FileFormat {
    id: 127_260_595,
    source_type: SourceType::Wikidata,
    name: "Abaqus/CAE model database",
    extensions: &["cae"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
