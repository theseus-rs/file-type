use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861047: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_047,
        source_type: SourceType::Wikidata,
        name: "Jynx Snapshot",
        extensions: &["lynxsnapshot"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3E, 0x63, 0x61, 0x6D, 0x70, 0x75, 0x74, 0x65, 0x72, 0x73, 0x5F, 0x6C,
                        0x79, 0x6E, 0x78,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
