use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122305680: FileFormat = FileFormat {
    id: 122_305_680,
    source_type: SourceType::Wikidata,
    name: "PressRelease Project",
    extensions: &["prp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
