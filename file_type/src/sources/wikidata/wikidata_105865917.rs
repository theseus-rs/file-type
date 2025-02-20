use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865917: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_917,
        source_type: SourceType::Wikidata,
        name: "Linux PC Screen Font data (PSF1)",
        extensions: &["psf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x36, 0x04])],
                },
            }],
        }],
        related_formats: &[],
    },
};
