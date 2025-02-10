use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105867486: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_486,
        source_type: SourceType::Wikidata,
        name: "Mario Kart DS track data",
        extensions: &["nkm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4E, 0x4B, 0x4D, 0x44])],
                },
            }],
        }],
        related_formats: &[],
    },
};
