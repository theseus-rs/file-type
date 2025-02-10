use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105862735: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_735,
        source_type: SourceType::Wikidata,
        name: "SciADV MPK game data Package",
        extensions: &["mpk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x50, 0x4B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
