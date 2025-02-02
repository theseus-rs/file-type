use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206455: FileFormat = FileFormat {
    id: 28_206_455,
    source_type: SourceType::Wikidata,
    name: "CKiSS",
    extensions: &["cel"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
