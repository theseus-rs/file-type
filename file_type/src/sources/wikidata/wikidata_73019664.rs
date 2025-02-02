use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_73019664: FileFormat = FileFormat {
    id: 73_019_664,
    source_type: SourceType::Wikidata,
    name: "WordPerfect for MS-DOS/Windows Document, version 5.1",
    extensions: &["wp5"],
    media_types: &["application/vnd.wordperfect"],
    internal_signatures: &[],
    related_formats: &[],
};
