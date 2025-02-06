use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2025: FileFormat = FileFormat {
    id: 2_025,
    source_type: SourceType::Pronom,
    name: "Reduced Resolution Dataset",
    extensions: &[
        "img", "ovr", "rrd", "aux", "aoi", "cff", "fft", "gcc", "sig", "sml",
    ],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x45, 0x48, 0x46, 0x41, 0x5F, 0x48, 0x45, 0x41, 0x44, 0x45, 0x52, 0x5F, 0x54,
                    0x41, 0x47, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
