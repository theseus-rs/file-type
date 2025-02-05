use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_129652416: FileFormat = FileFormat {
    id: 129_652_416,
    source_type: SourceType::Wikidata,
    name: "Inform 6 template file",
    extensions: &["i6t"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
