use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856884: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_884,
        source_type: SourceType::Wikidata,
        name: "Gerber format (with rem)",
        extensions: &["gbr"],
        media_types: &["application/vnd.gerber"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x30, 0x34, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
