use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856525: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_525,
        source_type: SourceType::Wikidata,
        name: "Spiderman 2 sound/music data",
        extensions: &["wbk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x57, 0x41, 0x56, 0x45, 0x42, 0x4B, 0x31, 0x31, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
