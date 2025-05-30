use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_73794456: FileType = FileType {
    file_format: &FileFormat {
        id: 73_794_456,
        source_type: SourceType::Wikidata,
        name: "Delphi QuickReport",
        extensions: &["qr2"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x6F, 0x62, 0x6A, 0x65, 0x63, 0x74, 0x20, 0x48, 0x65, 0x61, 0x64, 0x65,
                        0x72, 0x3A, 0x20, 0x54, 0x43, 0x6F, 0x6D, 0x70, 0x6F, 0x6E, 0x65, 0x6E,
                        0x74, 0x46, 0x69, 0x6C, 0x65, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
