use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858578: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_578,
        source_type: SourceType::Wikidata,
        name: "CHDK UBASIC script (with rem)",
        extensions: &["bas"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x72, 0x65, 0x6D, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
