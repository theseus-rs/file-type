use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850436: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_436,
        source_type: SourceType::Wikidata,
        name: "CFS compressed data",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xB9, 0xD9, 0x02, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
