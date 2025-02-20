use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862753: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_753,
        source_type: SourceType::Wikidata,
        name: "TMPEGEnc config file",
        extensions: &["mcf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x6F, 0x62, 0x6A, 0x65, 0x63, 0x74, 0x20, 0x54, 0x4D, 0x50, 0x45, 0x47,
                        0x43, 0x6F, 0x6E, 0x66, 0x69, 0x67, 0x46, 0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
