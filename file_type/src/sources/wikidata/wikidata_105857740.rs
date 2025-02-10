use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857740: FileFormat = FileFormat {
    id: 105_857_740,
    source_type: SourceType::Wikidata,
    name: "AccessData Encrypted disk image",
    extensions: &["ad1", "e01", "s01"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x44, 0x43, 0x52, 0x59, 0x50, 0x54, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
