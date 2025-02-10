use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28207313: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_313,
        source_type: SourceType::Wikidata,
        name: "Run length encoded True Colour Picture",
        extensions: &["tre"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x74, 0x72, 0x65, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
