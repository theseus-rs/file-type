use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853260: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_260,
        source_type: SourceType::Wikidata,
        name: "FileGateway Server configuration",
        extensions: &["prefs"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x52, 0x56])],
                },
            }],
        }],
        related_formats: &[],
    },
};
