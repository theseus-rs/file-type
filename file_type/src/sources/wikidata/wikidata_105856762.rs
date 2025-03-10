use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856762: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_762,
        source_type: SourceType::Wikidata,
        name: "Unity UnityArchive asset bundle",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x55, 0x6E, 0x69, 0x74, 0x79, 0x41, 0x72, 0x63, 0x68, 0x69, 0x76, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
