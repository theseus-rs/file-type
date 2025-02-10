use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850639: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_639,
        source_type: SourceType::Wikidata,
        name: "Consolidated Laser Ranging Prediction Format",
        extensions: &["cpf"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x31, 0x20, 0x43, 0x50, 0x46, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
