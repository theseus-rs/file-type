use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47539022: FileFormat = FileFormat {
    id: 47_539_022,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Drawing Standards File",
    extensions: &["dws"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
