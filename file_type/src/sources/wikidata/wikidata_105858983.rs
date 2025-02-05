use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858983: FileFormat = FileFormat {
    id: 105_858_983,
    source_type: SourceType::Wikidata,
    name: "Safari Cookies",
    extensions: &["binarycookies"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x63, 0x6F, 0x6F, 0x6B])],
            },
        }],
    }],
    related_formats: &[],
};
