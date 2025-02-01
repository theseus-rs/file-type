use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29208953: FileFormat = FileFormat {
    id: 29_208_953,
    puid: "wikidata/29208953",
    name: ".lzma File Format",
    extensions: &["lzma"],
    media_types: &["application/x-lzma"],
    internal_signatures: &[],
    related_formats: &[],
};
