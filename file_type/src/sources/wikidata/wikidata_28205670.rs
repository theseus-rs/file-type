use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205670: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_670,
        source_type: SourceType::Wikidata,
        name: "Alias PIX",
        extensions: &["als", "img", "pix"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x00, 0x00, 0x05, 0x00, 0x18])],
                },
            }],
        }],
        related_formats: &[],
    },
};
