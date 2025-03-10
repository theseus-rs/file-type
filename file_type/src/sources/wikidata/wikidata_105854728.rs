use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854728: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_728,
        source_type: SourceType::Wikidata,
        name: "Softdisk LIBrary game data archive",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x4C, 0x49, 0x42])],
                },
            }],
        }],
        related_formats: &[],
    },
};
