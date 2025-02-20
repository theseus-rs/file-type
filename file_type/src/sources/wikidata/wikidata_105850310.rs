use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850310: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_310,
        source_type: SourceType::Wikidata,
        name: "Grand Theft Auto 3 collision data",
        extensions: &["col"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x4F, 0x4C, 0x4C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
