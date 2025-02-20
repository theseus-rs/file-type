use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856150: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_150,
        source_type: SourceType::Wikidata,
        name: "Project Dogwaffle animation (type 1)",
        extensions: &["dwa"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x57, 0x41, 0x31, 0x20, 0x20, 0x20, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
