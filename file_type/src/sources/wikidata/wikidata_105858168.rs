use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858168: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_168,
        source_type: SourceType::Wikidata,
        name: "Embird Cross stitch Format",
        extensions: &["ecf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x71, 0x00, 0x00, 0x00, 0x18, 0x45, 0x6D, 0x62, 0x69, 0x72, 0x64, 0x20,
                        0x43, 0x72, 0x6F, 0x73, 0x73, 0x53, 0x74, 0x69, 0x74, 0x63, 0x68, 0x20,
                        0x76, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
