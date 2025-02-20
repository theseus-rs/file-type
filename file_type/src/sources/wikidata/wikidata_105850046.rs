use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850046: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_046,
        source_type: SourceType::Wikidata,
        name: "Windows Composite Font",
        extensions: &["compositefont"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xEF, 0xBB, 0xBF, 0x3C, 0x46, 0x6F, 0x6E, 0x74, 0x46, 0x61, 0x6D, 0x69,
                        0x6C, 0x79,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
