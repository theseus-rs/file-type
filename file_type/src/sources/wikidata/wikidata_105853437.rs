use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853437: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_437,
        source_type: SourceType::Wikidata,
        name: "Mr. Backup Z64 ROM dump",
        extensions: &["z64"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x80, 0x37, 0x12, 0x40, 0x00, 0x00, 0x00, 0x0F,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
