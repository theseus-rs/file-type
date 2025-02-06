use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61963212: FileFormat = FileFormat {
    id: 61_963_212,
    source_type: SourceType::Wikidata,
    name: "Lotus WordPro Document",
    extensions: &["lwp"],
    media_types: &["application/lwp", "application/vnd.lotus-wordpro"],
    signatures: &[],
    related_formats: &[],
};
