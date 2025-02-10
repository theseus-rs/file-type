use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861090: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_090,
        source_type: SourceType::Wikidata,
        name: "Livewire Document",
        extensions: &["lvw"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x73, 0x67, 0x00, 0x0A, 0x64, 0x0A, 0x00, 0x00, 0x01, 0x0A, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x6E, 0x0A, 0x00, 0x00, 0x00, 0x11,
                        0xB0, 0xBC, 0xB0, 0xBC, 0xB1, 0xAC, 0xAC,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
