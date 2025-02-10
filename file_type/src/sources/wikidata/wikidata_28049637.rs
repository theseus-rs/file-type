use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28049637: FileFormat = FileFormat {
    id: 28_049_637,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Slide",
    extensions: &["sld"],
    media_types: &[
        "application/octet-stream",
        "application/sld",
        "application/x-sld",
        "image/x-sld",
    ],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x75, 0x74, 0x6F, 0x43, 0x41, 0x44, 0x20, 0x53, 0x6C, 0x69, 0x64, 0x65,
                    0x0D, 0x0A, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
