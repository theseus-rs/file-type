use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_27979389: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_389,
        source_type: SourceType::Wikidata,
        name: "NEOchrome Animation",
        extensions: &["ani"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xBA, 0xBE, 0xEB, 0xEA])],
                },
            }],
        }],
        related_formats: &[],
    },
};
