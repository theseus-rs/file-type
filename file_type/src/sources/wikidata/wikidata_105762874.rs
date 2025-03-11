use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762874: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_874,
        source_type: SourceType::Wikidata,
        name: "XYPad Drawing",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x67, 0x72, 0x69, 0x64, 0x3A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
