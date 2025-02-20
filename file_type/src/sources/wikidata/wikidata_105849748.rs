use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849748: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_748,
        source_type: SourceType::Wikidata,
        name: "Opticks Camera",
        extensions: &["cam"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x0A, 0x43, 0x41, 0x4D, 0x45, 0x52, 0x41, 0x76, 0x31, 0x2E, 0x30, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
