use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_136442756: FileType = FileType {
    file_format: &FileFormat {
        id: 136_442_756,
        source_type: SourceType::Wikidata,
        name: "Textor document",
        extensions: &["TAL"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x01, 0x14, 0x00, 0x45, 0x54, 0x01, 0x02, 0x58, 0x54, 0x00, 0x06, 0x49,
                        0x57,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
