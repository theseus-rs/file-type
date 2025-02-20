use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850216: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_216,
        source_type: SourceType::Wikidata,
        name: "Midtown Madness 3 data",
        extensions: &["cdds"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xC8, 0x44, 0x0F, 0x99])],
                },
            }],
        }],
        related_formats: &[],
    },
};
