use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850939: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_939,
        source_type: SourceType::Wikidata,
        name: "TI-Nspire OS image",
        extensions: &["tno"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x49, 0x2D, 0x4E, 0x73, 0x70, 0x69, 0x72, 0x65, 0x2E, 0x74, 0x6E,
                        0x6F,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
