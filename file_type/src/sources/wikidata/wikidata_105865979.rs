use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865979: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_979,
        source_type: SourceType::Wikidata,
        name: "PiXCL text Palette",
        extensions: &["pal"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x69, 0x58, 0x43, 0x4C, 0x2D, 0x50, 0x41, 0x4C,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
