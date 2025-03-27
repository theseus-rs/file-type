use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_217570: FileType = FileType {
    file_format: &FileFormat {
        id: 217_570,
        source_type: SourceType::Wikidata,
        name: "Waveform Audio File Format",
        extensions: &["wav"],
        media_types: &["audio/vnd.wave"],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x57, 0x41, 0x56, 0x45])],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x52, 0x49, 0x46, 0x46]),
                            Token::WildcardCount(4),
                            Token::Literal(&[0x57, 0x41, 0x56, 0x45]),
                        ],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
