use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850377: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_377,
        source_type: SourceType::Wikidata,
        name: "ChessBase Archive file",
        extensions: &["cbv"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x08, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
