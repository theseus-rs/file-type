use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_73160459: FileFormat = FileFormat {
    id: 73_160_459,
    source_type: SourceType::Wikidata,
    name: "Visio Workspace",
    extensions: &["vsw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
