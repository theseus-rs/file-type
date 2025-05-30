use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854669: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_669,
        source_type: SourceType::Wikidata,
        name: "audfprint hash",
        extensions: &["afpt"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x61, 0x75, 0x64, 0x66, 0x70, 0x72, 0x69, 0x6E, 0x74, 0x68, 0x61, 0x73,
                        0x68, 0x56, 0x30, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
