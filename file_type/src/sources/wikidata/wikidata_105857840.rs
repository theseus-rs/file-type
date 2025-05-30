use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857840: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_840,
        source_type: SourceType::Wikidata,
        name: "Softbook eBook",
        extensions: &["imp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x02, 0x42, 0x4F, 0x4F, 0x4B, 0x44, 0x4F, 0x55, 0x47,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
