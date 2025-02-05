use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_73160459: FileFormat = FileFormat {
    id: 73_160_459,
    source_type: SourceType::Wikidata,
    name: "Visio Workspace",
    extensions: &["vsw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
