use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857410: FileFormat = FileFormat {
    id: 105_857_410,
    puid: "wikidata/105857410",
    name: "EditPad Pro Custom Syntax Coloring Scheme (UTF-8)",
    extensions: &["jgcscs"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xEF, 0xBB, 0xBF, 0x4A, 0x47, 0x43, 0x53, 0x43, 0x53,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
