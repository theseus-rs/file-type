use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111490198: FileType = FileType {
    file_format: &FileFormat {
        id: 111_490_198,
        source_type: SourceType::Wikidata,
        name: "Canvas Image File",
        extensions: &["cvi"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x41, 0x44, 0x35, 0x50, 0x52, 0x4F, 0x58,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
