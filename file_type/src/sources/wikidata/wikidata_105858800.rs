use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858800: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_800,
        source_type: SourceType::Wikidata,
        name: "PCX bitmap (v2.5)",
        extensions: &["pcx"],
        media_types: &["image/vnd.zbrush.pcx"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x0A, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
