use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855369: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_369,
        source_type: SourceType::Wikidata,
        name: "WinFax Pro multipage document",
        extensions: &["fxm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x0B, 0x23, 0xC8, 0xC0, 0x06])],
                },
            }],
        }],
        related_formats: &[],
    },
};
