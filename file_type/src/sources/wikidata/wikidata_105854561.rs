use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854561: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_561,
        source_type: SourceType::Wikidata,
        name: "ar archive",
        extensions: &["a", "ar", "lbr", "lib"],
        media_types: &["application/x-archive"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x21, 0x3C, 0x61, 0x72, 0x63, 0x68, 0x3E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
