use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862919: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_919,
        source_type: SourceType::Wikidata,
        name: "MVX Module",
        extensions: &["mvm"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x56, 0x4D, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
