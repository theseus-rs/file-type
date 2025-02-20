use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866968: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_968,
        source_type: SourceType::Wikidata,
        name: "NeroVision Express project",
        extensions: &["nvc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x4D, 0x43, 0x11, 0x08, 0x19, 0x77, 0x06, 0x7E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
