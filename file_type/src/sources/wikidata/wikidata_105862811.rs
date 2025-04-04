use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862811: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_811,
        source_type: SourceType::Wikidata,
        name: "MagicDraw UML project",
        extensions: &["mdr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x61, 0x67, 0x69, 0x63, 0x44, 0x72, 0x61, 0x77, 0x55, 0x4D, 0x4C,
                        0x20, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
