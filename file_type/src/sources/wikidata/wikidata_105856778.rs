use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856778: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_778,
        source_type: SourceType::Wikidata,
        name: "Ultra Fractal data and pixels",
        extensions: &["ufr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x55, 0x6C, 0x74, 0x72, 0x61, 0x46, 0x72, 0x61, 0x63, 0x74, 0x61, 0x6C,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
