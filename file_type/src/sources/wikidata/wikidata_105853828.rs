use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853828: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_828,
        source_type: SourceType::Wikidata,
        name: "Atari800Win Plus Trainer",
        extensions: &["a8t"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x38, 0x54, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
