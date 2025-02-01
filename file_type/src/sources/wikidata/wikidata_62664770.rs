use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_62664770: FileFormat = FileFormat {
    id: 62_664_770,
    puid: "wikidata/62664770",
    name: "WordPerfect for MS-DOS/Windows Document file format, version 6",
    extensions: &["doc", "w60", "wp", "wp6", "wpd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
