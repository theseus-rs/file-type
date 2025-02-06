use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127260495: FileFormat = FileFormat {
    id: 127_260_495,
    source_type: SourceType::Wikidata,
    name: "Abaqus output database",
    extensions: &["odb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
