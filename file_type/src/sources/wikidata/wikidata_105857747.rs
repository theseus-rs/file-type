use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857747: FileFormat = FileFormat {
    id: 105_857_747,
    source_type: SourceType::Wikidata,
    name: "EFI partition image",
    extensions: &["img"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x45, 0x46, 0x49, 0x20, 0x50, 0x41, 0x52, 0x54,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
