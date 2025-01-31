use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_50419912: FileFormat = FileFormat {
    id: 50_419_912,
    puid: "wikidata/50419912",
    name: "AutoShade Rendering Slide",
    extensions: &["rnd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x75, 0x74, 0x6F, 0x53, 0x68, 0x61, 0x64, 0x65, 0x20, 0x52, 0x65, 0x6E,
                    0x64, 0x65, 0x72, 0x69, 0x6E, 0x67, 0x20, 0x53, 0x6C, 0x69, 0x64, 0x65, 0x0D,
                    0x0A, 0x57, 0x72, 0x69, 0x74, 0x74, 0x65, 0x6E, 0x20, 0x62, 0x79, 0x3A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
