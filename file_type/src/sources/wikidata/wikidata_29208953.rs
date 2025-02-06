use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29208953: FileFormat = FileFormat {
    id: 29_208_953,
    source_type: SourceType::Wikidata,
    name: ".lzma File Format",
    extensions: &["lzma"],
    media_types: &["application/x-lzma"],
    signatures: &[],
    related_formats: &[],
};
