use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857629: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_629,
        source_type: SourceType::Wikidata,
        name: "InterSpread spreadsheet",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xFF, 0xB7, 0xB2, 0x01, 0xCE, 0xD1, 0xCF, 0xCF, 0xFF, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
