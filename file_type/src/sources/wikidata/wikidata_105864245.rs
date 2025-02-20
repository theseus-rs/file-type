use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864245: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_245,
        source_type: SourceType::Wikidata,
        name: "Palm Markup Language",
        extensions: &["pml"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5C, 0x76])],
                },
            }],
        }],
        related_formats: &[],
    },
};
