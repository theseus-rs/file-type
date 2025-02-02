use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122305680: FileFormat = FileFormat {
    id: 122_305_680,
    source_type: SourceType::Wikidata,
    name: "PressRelease Project",
    extensions: &["prp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
