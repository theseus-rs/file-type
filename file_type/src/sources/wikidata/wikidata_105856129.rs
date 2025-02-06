use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856129: FileFormat = FileFormat {
    id: 105_856_129,
    source_type: SourceType::Wikidata,
    name: "Delphi Project source (with rem)",
    extensions: &["dpr"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
