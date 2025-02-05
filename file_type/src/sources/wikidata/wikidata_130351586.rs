use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130351586: FileFormat = FileFormat {
    id: 130_351_586,
    source_type: SourceType::Wikidata,
    name: "Monkey source code file",
    extensions: &["monkey"],
    media_types: &["text/x-monkey"],
    signatures: &[],
    related_formats: &[],
};
