use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858923: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_923,
        source_type: SourceType::Wikidata,
        name: "Buzzic 2 module",
        extensions: &["buz2"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x62, 0x75, 0x7A, 0x32])],
                },
            }],
        }],
        related_formats: &[],
    },
};
