use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855061: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_061,
        source_type: SourceType::Wikidata,
        name: "SBC compressed archive",
        extensions: &["sbc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x42, 0x43, 0x1E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
