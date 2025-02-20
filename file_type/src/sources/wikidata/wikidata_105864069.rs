use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864069: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_069,
        source_type: SourceType::Wikidata,
        name: "PlayStation RSD Material (gen)",
        extensions: &["mat"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x40, 0x4D, 0x41, 0x54])],
                },
            }],
        }],
        related_formats: &[],
    },
};
