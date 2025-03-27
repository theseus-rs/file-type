use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28830544: FileType = FileType {
    file_format: &FileFormat {
        id: 28_830_544,
        source_type: SourceType::Wikidata,
        name: "Adobe filmstrip",
        extensions: &["flm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x61, 0x6E, 0x64])],
                },
            }],
        }],
        related_formats: &[],
    },
};
