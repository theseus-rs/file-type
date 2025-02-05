use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_121814737: FileFormat = FileFormat {
    id: 121_814_737,
    source_type: SourceType::Wikidata,
    name: "Common Interface File",
    extensions: &["cif", "mca"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
