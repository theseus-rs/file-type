use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_27979366: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_366,
        source_type: SourceType::Wikidata,
        name: "Flash Media Manifest",
        extensions: &["f4m"],
        media_types: &["application/f4m"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
