use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2331: FileType = FileType {
    file_format: &FileFormat {
        id: 2_331,
        source_type: SourceType::Pronom,
        name: "Microsoft Visio Drawing",
        extensions: &["vsd", "vst", "vss"],
        media_types: &["application/vnd.visio"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x56, 0x69, 0x73, 0x69, 0x6F, 0x20, 0x28, 0x54, 0x4D, 0x29, 0x20, 0x44,
                            0x72, 0x61, 0x77, 0x69, 0x6E, 0x67, 0x0D, 0x0A,
                        ]),
                        Token::WildcardCount(6),
                        Token::Literal(&[0x02]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
