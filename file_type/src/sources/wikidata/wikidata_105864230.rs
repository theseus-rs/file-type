use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105864230: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_230,
        source_type: SourceType::Wikidata,
        name: "PowerBASIC Help",
        extensions: &["pbh"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x72, 0x7A, 0x6A, 0x66, 0x68, 0x65, 0x6C, 0x70,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
