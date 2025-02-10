use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105862769: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_769,
        source_type: SourceType::Wikidata,
        name: "OctaMED Music Editor module (v2.10)",
        extensions: &["med4"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x45, 0x44, 0x04])],
                },
            }],
        }],
        related_formats: &[],
    },
};
