use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865798: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_798,
        source_type: SourceType::Wikidata,
        name: "Chief Architect plan",
        extensions: &["plan"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x01, 0xCA])],
                },
            }],
        }],
        related_formats: &[],
    },
};
