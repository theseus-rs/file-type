use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29650512: FileFormat = FileFormat {
    id: 29_650_512,
    source_type: SourceType::Wikidata,
    name: "packJPG",
    extensions: &["pjg"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
