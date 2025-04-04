use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855089: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_089,
        source_type: SourceType::Wikidata,
        name: "League Of Legends bones based Animation",
        extensions: &["anm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x72, 0x33, 0x64, 0x32, 0x61, 0x6E, 0x6D, 0x64,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
