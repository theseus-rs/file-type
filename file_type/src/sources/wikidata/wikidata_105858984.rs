use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858984: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_984,
        source_type: SourceType::Wikidata,
        name: "Wwise sound Bank",
        extensions: &["bnk"],
        media_types: &["application/vnd.wwise.bnk"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x4B, 0x48, 0x44])],
                },
            }],
        }],
        related_formats: &[],
    },
};
