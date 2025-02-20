use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205464: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_464,
        source_type: SourceType::Wikidata,
        name: "PSF",
        extensions: &["psf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x53, 0x50, 0x41])],
                },
            }],
        }],
        related_formats: &[],
    },
};
