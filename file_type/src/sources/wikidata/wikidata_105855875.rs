use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855875: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_875,
        source_type: SourceType::Wikidata,
        name: "Psion Archive Data Base",
        extensions: &["dbf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x62, 0x72, 0x6D, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
