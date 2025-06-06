use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865946: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_946,
        source_type: SourceType::Wikidata,
        name: "ProCite data (v5+",
        extensions: &["pdt"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xB4, 0xF4, 0x5C, 0x00, 0x0C, 0xC1, 0x9A, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
