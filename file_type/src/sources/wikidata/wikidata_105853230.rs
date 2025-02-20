use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853230: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_230,
        source_type: SourceType::Wikidata,
        name: "Sony Foundry Video Capture project",
        extensions: &["sfvidcap"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x72, 0x69, 0x66, 0x66])],
                },
            }],
        }],
        related_formats: &[],
    },
};
