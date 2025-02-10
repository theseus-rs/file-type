use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850758: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_758,
        source_type: SourceType::Wikidata,
        name: "Suncom F-15E Eagle Keys config",
        extensions: &["key"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x01, 0x02, 0x00, 0x1B, 0x00, 0x46, 0x31, 0x35, 0x20, 0x45, 0x61,
                        0x67, 0x6C, 0x65, 0x20, 0x53, 0x65, 0x74, 0x75, 0x70, 0x20, 0x50, 0x72,
                        0x6F, 0x67, 0x72, 0x61, 0x6D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
