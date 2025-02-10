use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28207336: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_336,
        source_type: SourceType::Wikidata,
        name: "Sun TAAC image",
        extensions: &["iff", "suniff", "taac", "vff"],
        media_types: &["image/x-vff"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x6E, 0x63, 0x61, 0x61])],
                },
            }],
        }],
        related_formats: &[],
    },
};
