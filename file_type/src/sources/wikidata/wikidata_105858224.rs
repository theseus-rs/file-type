use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858224: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_224,
        source_type: SourceType::Wikidata,
        name: "Entrust Entelligence Profile",
        extensions: &["epf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x50, 0x61, 0x73, 0x73, 0x77, 0x6F, 0x72, 0x64, 0x20, 0x54, 0x6F,
                        0x6B, 0x65, 0x6E, 0x5D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
