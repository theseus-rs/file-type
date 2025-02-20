use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854808: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_808,
        source_type: SourceType::Wikidata,
        name: "SBX SpinnerBaker eXtractor compressed archive",
        extensions: &["sb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x42, 0x31, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
