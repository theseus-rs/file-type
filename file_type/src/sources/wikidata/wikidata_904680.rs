use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_904680: FileType = FileType {
    file_format: &FileFormat {
        id: 904_680,
        source_type: SourceType::Wikidata,
        name: "Multiple-image Network Graphics",
        extensions: &["mng"],
        media_types: &["image/x-jng", "image/x-mng", "video/x-mng"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x8A, 0x4D, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x1C,
                        0x4D, 0x48, 0x44, 0x52,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
