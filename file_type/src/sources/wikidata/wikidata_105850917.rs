use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850917: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_917,
        source_type: SourceType::Wikidata,
        name: "TurboCalc Document",
        extensions: &["tcd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x01, 0x00, 0x0C, 0x54, 0x55, 0x52, 0x42, 0x4F, 0x43, 0x41, 0x4C, 0x43,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
