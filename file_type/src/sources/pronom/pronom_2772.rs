use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2772: FileType = FileType {
    file_format: &FileFormat {
        id: 2_772,
        source_type: SourceType::Pronom,
        name: "Graphisoft BIMx Hyper-Model",
        extensions: &["bimx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x42, 0x49, 0x4D, 0x78])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x4B, 0x50, 0x58, 0x42])],
                    },
                },
            ],
        }],
        related_formats: &[],
    },
};
