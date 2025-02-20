use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856786: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_786,
        source_type: SourceType::Wikidata,
        name: "GDLib Image",
        extensions: &["gd2"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x67, 0x64, 0x32])],
                },
            }],
        }],
        related_formats: &[],
    },
};
