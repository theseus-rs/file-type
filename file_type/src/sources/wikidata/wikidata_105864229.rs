use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864229: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_229,
        source_type: SourceType::Wikidata,
        name: "SpeedScript document (C64)",
        extensions: &["prg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x1C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
