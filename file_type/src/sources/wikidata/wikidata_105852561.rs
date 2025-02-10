use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852561: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_561,
        source_type: SourceType::Wikidata,
        name: "Siemens mobile theme",
        extensions: &["sdt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x4B, 0x03])],
                },
            }],
        }],
        related_formats: &[],
    },
};
