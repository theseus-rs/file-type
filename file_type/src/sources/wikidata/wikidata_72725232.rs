use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_72725232: FileType = FileType {
    file_format: &FileFormat {
        id: 72_725_232,
        source_type: SourceType::Wikidata,
        name: "NOD32 Antivirus Update file",
        extensions: &["nup"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x90, 0x0D, 0x03, 0x00, 0x02, 0x11, 0xC8, 0xFC, 0xA0, 0x02, 0x06, 0x7B,
                        0x03, 0xC9, 0x01,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
