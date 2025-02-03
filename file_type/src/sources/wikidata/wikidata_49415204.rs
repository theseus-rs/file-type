use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_49415204: FileFormat = FileFormat {
    id: 49_415_204,
    source_type: SourceType::Wikidata,
    name: "CATIA Project, version 4",
    extensions: &["project"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
