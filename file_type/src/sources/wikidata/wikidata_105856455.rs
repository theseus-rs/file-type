use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856455: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_455,
        source_type: SourceType::Wikidata,
        name: "Yokogawa waveform data",
        extensions: &["wvf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x25, 0x57, 0x44, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
