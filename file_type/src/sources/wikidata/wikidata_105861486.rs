use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861486: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_486,
        source_type: SourceType::Wikidata,
        name: "Micro Focus VS Cobol library",
        extensions: &["lbr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x69, 0x63, 0x72, 0x6F, 0x20, 0x46, 0x6F, 0x63, 0x75, 0x73, 0x20,
                        0x56, 0x53, 0x20, 0x43, 0x6F, 0x62, 0x6F, 0x6C, 0x20, 0x4C, 0x69, 0x62,
                        0x72, 0x61, 0x72, 0x79, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
