use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858560: FileFormat = FileFormat {
    id: 105_858_560,
    source_type: SourceType::Wikidata,
    name: "PrintFox/Pagefox bitmap (640x400)",
    extensions: &["bg", "bin"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
