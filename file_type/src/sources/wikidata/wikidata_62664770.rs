use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_62664770: FileFormat = FileFormat {
    id: 62_664_770,
    source_type: SourceType::Wikidata,
    name: "WordPerfect for MS-DOS/Windows Document file format, version 6",
    extensions: &["doc", "w60", "wp", "wp6", "wpd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
