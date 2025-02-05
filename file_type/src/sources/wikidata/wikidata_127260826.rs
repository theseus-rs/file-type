use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127260826: FileFormat = FileFormat {
    id: 127_260_826,
    source_type: SourceType::Wikidata,
    name: "Abaqus substructure file",
    extensions: &["sim"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
