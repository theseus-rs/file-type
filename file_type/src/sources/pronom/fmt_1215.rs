use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1215: FileFormat = FileFormat {
    id: 2_025,
    puid: "fmt/1215",
    name: "Reduced Resolution Dataset",
    extensions: &[
        "img", "ovr", "rrd", "aux", "aoi", "cff", "fft", "gcc", "sig", "sml",
    ],
    media_types: &[],
    internal_signatures: &[InternalSignature {
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
