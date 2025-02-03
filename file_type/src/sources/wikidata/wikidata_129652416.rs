use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_129652416: FileFormat = FileFormat {
    id: 129_652_416,
    source_type: SourceType::Wikidata,
    name: "Inform 6 template file",
    extensions: &["i6t"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
