use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762658: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_658,
        source_type: SourceType::Wikidata,
        name: "Biew Xlat Table",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x69, 0x65, 0x77, 0x20, 0x58, 0x6C, 0x61, 0x74, 0x20, 0x54, 0x61,
                        0x62, 0x6C, 0x65, 0x2E, 0x20, 0x43, 0x6F, 0x6E, 0x76, 0x65, 0x72, 0x74,
                        0x73, 0x20, 0x66, 0x72, 0x6F, 0x6D, 0x20, 0x72, 0x75, 0x73, 0x73, 0x69,
                        0x61, 0x6E, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
