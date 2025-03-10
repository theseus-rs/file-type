use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762912: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_912,
        source_type: SourceType::Wikidata,
        name: "XPCOM Type Library",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x58, 0x50, 0x43, 0x4F, 0x4D, 0x0A, 0x54, 0x79, 0x70, 0x65, 0x4C, 0x69,
                        0x62,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
