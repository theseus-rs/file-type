use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_27979555: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_555,
        source_type: SourceType::Wikidata,
        name: "Civilization III BIX saved game format",
        extensions: &["bix"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x49, 0x43, 0x58, 0x56, 0x45, 0x52, 0x23,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
