use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130750886: FileFormat = FileFormat {
    id: 130_750_886,
    source_type: SourceType::Wikidata,
    name: "Sed script file",
    extensions: &["sed"],
    media_types: &["text/x-sed"],
    signatures: &[],
    related_formats: &[],
};
