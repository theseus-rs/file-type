use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862136: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_136,
        source_type: SourceType::Wikidata,
        name: "MusicMaker Instrument",
        extensions: &["ip"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x45, 0x49, 0x31, 0x58, 0x58, 0x00, 0x24,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
