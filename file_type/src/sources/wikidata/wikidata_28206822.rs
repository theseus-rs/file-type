use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206822: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_822,
        source_type: SourceType::Wikidata,
        name: "PaintShop Pro image",
        extensions: &["jsl", "pfr", "psp", "pspbrush", "pspimage", "tub"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7E, 0x42, 0x4B, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
