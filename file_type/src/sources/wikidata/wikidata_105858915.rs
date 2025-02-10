use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858915: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_915,
        source_type: SourceType::Wikidata,
        name: "VDC BitMap (v3)",
        extensions: &["bm", "vbm"],
        media_types: &["image/x-commodore-vbm"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x42, 0x4D, 0xCB, 0x03])],
                },
            }],
        }],
        related_formats: &[],
    },
};
