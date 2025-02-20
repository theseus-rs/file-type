use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865835: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_835,
        source_type: SourceType::Wikidata,
        name: "PSP metadata file",
        extensions: &["psf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x50, 0x53, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
