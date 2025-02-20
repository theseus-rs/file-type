use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205588: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_588,
        source_type: SourceType::Wikidata,
        name: "PaintShop Pro Browser Cache",
        extensions: &["jbf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4A, 0x41, 0x53, 0x43, 0x20, 0x42, 0x52, 0x4F, 0x57, 0x53, 0x20, 0x46,
                        0x49, 0x4C, 0x45,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
