use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862687: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_687,
        source_type: SourceType::Wikidata,
        name: "MS-DOS Backup Control data (v3.30-5.xx)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x8B, 0x42, 0x41, 0x43, 0x4B, 0x55, 0x50, 0x20, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
