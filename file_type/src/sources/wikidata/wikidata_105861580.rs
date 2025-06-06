use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861580: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_580,
        source_type: SourceType::Wikidata,
        name: "DipTrace Pattern Library",
        extensions: &["lib"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x06, 0x44, 0x54, 0x43, 0x4C, 0x49, 0x42, 0x0F, 0x42,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
