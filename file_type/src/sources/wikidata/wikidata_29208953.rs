use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29208953: FileFormat = FileFormat {
    id: 29_208_953,
    source_type: SourceType::Wikidata,
    name: ".lzma File Format",
    extensions: &["lzma"],
    media_types: &["application/x-lzma"],
    internal_signatures: &[],
    related_formats: &[],
};
