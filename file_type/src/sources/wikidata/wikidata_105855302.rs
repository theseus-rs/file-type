use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855302: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_302,
        source_type: SourceType::Wikidata,
        name: "Far Cry 3 map",
        extensions: &["fc3map"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x13, 0x00, 0x00, 0x00, 0x6B, 0x0A, 0xFD, 0xD2,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
