use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206455: FileFormat = FileFormat {
    id: 28_206_455,
    source_type: SourceType::Wikidata,
    name: "CKiSS",
    extensions: &["cel"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
