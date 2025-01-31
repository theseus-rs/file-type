use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858229: FileFormat = FileFormat {
    id: 105_858_229,
    puid: "wikidata/105858229",
    name: "EQATEC Profiler configuration",
    extensions: &["eqconfig"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xEF, 0xBB, 0xBF, 0x3C, 0x63, 0x6F, 0x6E, 0x66, 0x69, 0x67,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
