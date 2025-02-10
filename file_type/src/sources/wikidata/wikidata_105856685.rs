use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856685: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_685,
        source_type: SourceType::Wikidata,
        name: "Unreal Texture",
        extensions: &["utx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xC1, 0x83, 0x2A, 0x9E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
