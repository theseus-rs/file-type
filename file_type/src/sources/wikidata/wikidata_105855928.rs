use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855928: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_928,
        source_type: SourceType::Wikidata,
        name: "Colin McRae DiRT game data archive",
        extensions: &["dic"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x49, 0x43, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
