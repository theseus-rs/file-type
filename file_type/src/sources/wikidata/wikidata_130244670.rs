use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130244670: FileFormat = FileFormat {
    id: 130_244_670,
    source_type: SourceType::Wikidata,
    name: "LiveScript source code file",
    extensions: &["ls"],
    media_types: &["text/livescript"],
    signatures: &[],
    related_formats: &[],
};
