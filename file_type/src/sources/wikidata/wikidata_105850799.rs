use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850799: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_799,
        source_type: SourceType::Wikidata,
        name: "Microsoft Keyboard Layout Creator source (UTF-16-LE)",
        extensions: &["klc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xFF, 0xFE, 0x4B, 0x00, 0x42, 0x00, 0x44, 0x00, 0x09, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
