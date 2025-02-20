use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864294: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_294,
        source_type: SourceType::Wikidata,
        name: "PSF Playstation Sound Format rip",
        extensions: &["psf", "psflib"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x53, 0x46, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
