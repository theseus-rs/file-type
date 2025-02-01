use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864513: FileFormat = FileFormat {
    id: 105_864_513,
    puid: "wikidata/105864513",
    name: "MATLAB Compiler Project",
    extensions: &["prj"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x64, 0x65, 0x70, 0x6C, 0x6F, 0x79, 0x6D, 0x65, 0x6E, 0x74, 0x2D, 0x70,
                    0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74, 0x20, 0x70, 0x6C, 0x75, 0x67, 0x69, 0x6E,
                    0x3D, 0x22, 0x70, 0x6C, 0x75, 0x67, 0x69, 0x6E, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
