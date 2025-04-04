use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856716: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_716,
        source_type: SourceType::Wikidata,
        name: "HxC Floppy Emulator firmware Update (std)",
        extensions: &["upd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x48, 0x58, 0x43, 0x46, 0x55, 0x50, 0x44, 0x41, 0x54, 0x45, 0x00, 0x03,
                        0x00, 0x53, 0x54, 0x46, 0x45,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
