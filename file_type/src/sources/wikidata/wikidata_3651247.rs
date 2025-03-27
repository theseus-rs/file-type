use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_3651247: FileType = FileType {
    file_format: &FileFormat {
        id: 3_651_247,
        source_type: SourceType::Wikidata,
        name: "Camera Image File Format",
        extensions: &["crw"],
        media_types: &["image/x-canon-crw"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x49, 0x49, 0x1A, 0x00, 0x00, 0x00, 0x48, 0x45, 0x41, 0x50, 0x43, 0x43,
                        0x44, 0x52, 0x02, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
