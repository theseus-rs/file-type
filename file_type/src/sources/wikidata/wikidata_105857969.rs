use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857969: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_969,
        source_type: SourceType::Wikidata,
        name: "IncrediMail sound (MIDI music)",
        extensions: &["imw"],
        media_types: &["application/vnd.ms-cab-compressed"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x53, 0x43, 0x46, 0x00, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
