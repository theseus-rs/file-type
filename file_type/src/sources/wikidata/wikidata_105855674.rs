use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855674: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_674,
        source_type: SourceType::Wikidata,
        name: "Stanford Research Systems Optimlelt file",
        extensions: &["op2"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x74, 0x61, 0x6E, 0x66, 0x6F, 0x72, 0x64, 0x20, 0x52, 0x65, 0x73,
                        0x65, 0x61, 0x72, 0x63, 0x68, 0x20, 0x53, 0x79, 0x73, 0x74, 0x65, 0x6D,
                        0x73, 0x20, 0x4F, 0x70, 0x74, 0x69, 0x6D, 0x6C, 0x65, 0x6C, 0x74, 0x20,
                        0x46, 0x69, 0x6C, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
