use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130601735: FileFormat = FileFormat {
    id: 130_601_735,
    source_type: SourceType::Wikidata,
    name: "R console transcript file",
    extensions: &["rout"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
