use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861168: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_168,
        source_type: SourceType::Wikidata,
        name: "Amiga WHDLoad package (lha compressed)",
        extensions: &["lha"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2D, 0x6C, 0x68, 0x35, 0x2D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
