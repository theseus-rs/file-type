use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860615: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_615,
        source_type: SourceType::Wikidata,
        name: "Philips Respironics M-Series data format",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x49])],
                },
            }],
        }],
        related_formats: &[],
    },
};
