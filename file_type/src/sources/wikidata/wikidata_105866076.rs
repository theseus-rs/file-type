use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866076: FileFormat = FileFormat {
    id: 105_866_076,
    source_type: SourceType::Wikidata,
    name: "PSF2 Playstation 2 Sound Format rip",
    extensions: &["psf2", "psf2lib"],
    media_types: &["application/octet-stream"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x53, 0x46, 0x02])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x53, 0x46, 0x02])],
                },
            }],
        },
    ],
    related_formats: &[],
};
