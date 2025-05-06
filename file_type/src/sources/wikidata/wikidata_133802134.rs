use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_133802134: FileType = FileType {
    file_format: &FileFormat {
        id: 133_802_134,
        source_type: SourceType::Wikidata,
        name: "Fontographer Document",
        extensions: &["fog"],
        media_types: &[],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x43, 0x66, 0x66, 0x32])],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x43, 0x66, 0x66, 0x31])],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
