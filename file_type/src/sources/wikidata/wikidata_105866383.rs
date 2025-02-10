use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105866383: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_383,
        source_type: SourceType::Wikidata,
        name: "Pocket Word document (v1)",
        extensions: &["psw", "pwd"],
        media_types: &["application/x-pocket-word"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7B, 0x5C, 0x70, 0x77, 0x64, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
