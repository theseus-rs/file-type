use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856615: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_615,
        source_type: SourceType::Wikidata,
        name: "WiX Project",
        extensions: &["wixproj"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
