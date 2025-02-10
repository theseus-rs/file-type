use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_27967393: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_393,
        source_type: SourceType::Wikidata,
        name: "Adlib Tracker II instrument bank with macros",
        extensions: &["a2w"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x32, 0x69, 0x6E, 0x73, 0x62, 0x61, 0x6E, 0x6B, 0x5F, 0x77, 0x2F, 0x6D,
                        0x61, 0x63, 0x72, 0x6F, 0x73, 0x5F,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
