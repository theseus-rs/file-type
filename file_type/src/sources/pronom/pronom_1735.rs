use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1735: FileType = FileType {
    file_format: &FileFormat {
        id: 1_735,
        source_type: SourceType::Pronom,
        name: "Magick Image File Format",
        extensions: &["mif", "miff"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x69, 0x64, 0x3D, 0x49, 0x6D, 0x61, 0x67, 0x65, 0x4D, 0x61, 0x67, 0x69,
                            0x63, 0x6B,
                        ]),
                        Token::WildcardCountRange(0, 256),
                        Token::Literal(&[0x3A, 0x0A]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
