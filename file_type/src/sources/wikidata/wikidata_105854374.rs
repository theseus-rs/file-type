use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854374: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_374,
        source_type: SourceType::Wikidata,
        name: "Aladdin 4D ATList",
        extensions: &["atl"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x41, 0x54, 0x4C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
