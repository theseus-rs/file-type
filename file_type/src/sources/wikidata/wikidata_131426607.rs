use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131426607: FileFormat = FileFormat {
    id: 131_426_607,
    source_type: SourceType::Wikidata,
    name: "Wren source code file format",
    extensions: &["wren"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
