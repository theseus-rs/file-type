use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853539: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_539,
        source_type: SourceType::Wikidata,
        name: "z-Tree Questionnaire",
        extensions: &["ztq"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x45, 0x45, 0x58, 0x51, 0x75, 0x65, 0x73, 0x74, 0x65, 0x72,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
