use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855583: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_583,
        source_type: SourceType::Wikidata,
        name: "World Construction Set Object",
        extensions: &["obj"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x57, 0x43, 0x53, 0x56, 0x65, 0x63, 0x74, 0x6F, 0x72, 0x3F,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
