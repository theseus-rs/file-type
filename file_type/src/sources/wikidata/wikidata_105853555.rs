use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853555: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_555,
        source_type: SourceType::Wikidata,
        name: "Zeppelin ransomware encrypted",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xDA, 0xC4, 0xC4, 0xC4, 0xC4, 0xC4, 0xC4, 0xC4, 0xC4, 0xBF, 0x0D, 0x0A,
                        0xB3, 0x5A, 0x45, 0x50, 0x50, 0x45, 0x4C, 0x49, 0x4E, 0xB3, 0x0D, 0x0A,
                        0xC0, 0xC4, 0xC4, 0xC4, 0xC4, 0xC4, 0xC4, 0xC4, 0xC4, 0xD9, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
