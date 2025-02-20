use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853998: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_998,
        source_type: SourceType::Wikidata,
        name: "ATF Texture",
        extensions: &["atf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x54, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
