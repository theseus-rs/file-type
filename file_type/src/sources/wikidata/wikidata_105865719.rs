use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865719: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_719,
        source_type: SourceType::Wikidata,
        name: "PowerArchiver Rijndael Encrypted file",
        extensions: &["pae"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x69, 0x6A, 0x6E, 0x64, 0x61, 0x65, 0x6C, 0x20, 0x45, 0x6E, 0x63,
                        0x72, 0x79, 0x70, 0x74, 0x65, 0x64, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x20,
                        0x28, 0x50, 0x6F, 0x77, 0x65, 0x72, 0x41, 0x72, 0x63, 0x68, 0x69, 0x76,
                        0x65, 0x72, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
