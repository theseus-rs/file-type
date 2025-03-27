use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1341482: FileType = FileType {
    file_format: &FileFormat {
        id: 1_341_482,
        source_type: SourceType::Wikidata,
        name: "OpenEXR",
        extensions: &["exr"],
        media_types: &["image/x-exr"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x76, 0x2F, 0x31, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
