use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859215: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_215,
        source_type: SourceType::Wikidata,
        name: "Pervasive Btrieve (generic)",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x43, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
