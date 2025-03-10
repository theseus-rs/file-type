use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853076: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_076,
        source_type: SourceType::Wikidata,
        name: "ScriptBasic intermediate code",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x34, 0x42, 0x41, 0x53, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
