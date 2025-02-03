use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_127260495: FileFormat = FileFormat {
    id: 127_260_495,
    source_type: SourceType::Wikidata,
    name: "Abaqus output database",
    extensions: &["odb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
