use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852389: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_389,
        source_type: SourceType::Wikidata,
        name: "Roland EASE Song (v1.0)",
        extensions: &["sng"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xFD, 0x46, 0x51, 0x31, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x99,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x64, 0x64, 0x64, 0x64,
                        0x64, 0x64, 0x64, 0x64, 0x64, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x78,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
