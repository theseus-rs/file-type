use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_119786488: FileFormat = FileFormat {
    id: 119_786_488,
    source_type: SourceType::Wikidata,
    name: "MasterCook Export File",
    extensions: &["mx2"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
