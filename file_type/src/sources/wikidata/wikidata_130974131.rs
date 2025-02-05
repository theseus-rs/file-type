use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130974131: FileFormat = FileFormat {
    id: 130_974_131,
    source_type: SourceType::Wikidata,
    name: "Silver source code file",
    extensions: &["sil", "ver"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
