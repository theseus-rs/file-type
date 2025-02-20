use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_918221: FileType = FileType {
    file_format: &FileFormat {
        id: 918_221,
        source_type: SourceType::Wikidata,
        name: "Web Open Font Format",
        extensions: &["woff"],
        media_types: &["font/woff"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x77, 0x4F, 0x46, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
