use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130057181: FileFormat = FileFormat {
    id: 130_057_181,
    source_type: SourceType::Wikidata,
    name: "K source code file",
    extensions: &["k"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
