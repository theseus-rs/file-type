use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131426607: FileFormat = FileFormat {
    id: 131_426_607,
    source_type: SourceType::Wikidata,
    name: "Wren source code file format",
    extensions: &["wren"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
