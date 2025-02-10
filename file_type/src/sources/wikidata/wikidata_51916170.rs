use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_51916170: FileType = FileType {
    file_format: &FileFormat {
        id: 51_916_170,
        source_type: SourceType::Wikidata,
        name: "Pagemaker TableEditor Graphics",
        extensions: &["tbl"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x00, 0x01, 0x00, 0x02, 0x00, 0x01, 0x4B, 0x02, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
