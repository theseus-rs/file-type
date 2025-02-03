use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_121814737: FileFormat = FileFormat {
    id: 121_814_737,
    source_type: SourceType::Wikidata,
    name: "Common Interface File",
    extensions: &["cif", "mca"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
