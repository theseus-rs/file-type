use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852355: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_355,
        source_type: SourceType::Wikidata,
        name: "Seeyon encripted attachment",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x73, 0x65, 0x65, 0x79, 0x6F, 0x6E, 0x20, 0x61, 0x74, 0x74, 0x61, 0x63,
                        0x68, 0x6D, 0x65, 0x6E, 0x74, 0x20, 0x65, 0x6E, 0x63, 0x72, 0x79, 0x70,
                        0x74, 0x20, 0x76,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
