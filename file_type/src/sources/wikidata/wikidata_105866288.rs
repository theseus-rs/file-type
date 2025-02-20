use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866288: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_288,
        source_type: SourceType::Wikidata,
        name: "Pksmart compressed data (v1)",
        extensions: &["pks"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x4B, 0x53, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
