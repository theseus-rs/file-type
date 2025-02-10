use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105862058: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_058,
        source_type: SourceType::Wikidata,
        name: "Macadam Bumper pinball",
        extensions: &["mc1", "mc2", "mc3"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x48, 0x42])],
                },
            }],
        }],
        related_formats: &[],
    },
};
