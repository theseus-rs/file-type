use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864075: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_075,
        source_type: SourceType::Wikidata,
        name: "Scrull Music File",
        extensions: &["smf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x63, 0x72, 0x75, 0x6C, 0x6C, 0x20, 0x6D, 0x75, 0x73, 0x69, 0x63,
                        0x20, 0x66, 0x69, 0x6C, 0x65, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
