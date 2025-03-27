use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_877050: FileType = FileType {
    file_format: &FileFormat {
        id: 877_050,
        source_type: SourceType::Wikidata,
        name: "optical disc image",
        extensions: &["img", "iso"],
        media_types: &["application/vnd.efi.iso", "application/x-iso9660-image"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x6C, 0x69, 0x6E, 0x6B, 0x20, 0x62, 0x79, 0x20, 0x44, 0x2E, 0x54,
                        0x2E, 0x53,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
