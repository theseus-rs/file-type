use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865767: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_767,
        source_type: SourceType::Wikidata,
        name: "IBM i Access Client Poppad",
        extensions: &["pmp"],
        media_types: &["text/ini"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x50, 0x72, 0x6F, 0x66, 0x69, 0x6C, 0x65, 0x5D, 0x0D, 0x0A, 0x69,
                        0x64, 0x3D, 0x50, 0x4D, 0x50, 0x0D, 0x0A, 0x44, 0x65, 0x73, 0x63, 0x72,
                        0x69, 0x70, 0x74, 0x69, 0x6F, 0x6E, 0x3D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
