use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105864847: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_847,
        source_type: SourceType::Wikidata,
        name: "PI Image Palette",
        extensions: &["pal"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x4F, 0x52, 0x4D, 0x50, 0x41, 0x50, 0x49,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
