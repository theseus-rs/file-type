use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861131: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_131,
        source_type: SourceType::Wikidata,
        name: "Lazarus Form",
        extensions: &["lfm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x6F, 0x62, 0x6A, 0x65, 0x63, 0x74, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
