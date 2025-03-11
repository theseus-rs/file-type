use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850057: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_057,
        source_type: SourceType::Wikidata,
        name: "ChupaText encrypted data",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x43, 0x44, 0x53, 0x41, 0x30, 0x30, 0x32, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
