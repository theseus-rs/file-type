use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850084: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_084,
        source_type: SourceType::Wikidata,
        name: "Windows FAX cover",
        extensions: &["cpe"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x41, 0x58, 0x43, 0x4F, 0x56, 0x45, 0x52, 0x2D, 0x56, 0x45, 0x52,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
