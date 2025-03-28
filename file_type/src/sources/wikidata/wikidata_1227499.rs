use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1227499: FileType = FileType {
    file_format: &FileFormat {
        id: 1_227_499,
        source_type: SourceType::Wikidata,
        name: "Direct Stream Digital",
        extensions: &["dsf"],
        media_types: &["audio/x-dsf"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x53, 0x44, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
