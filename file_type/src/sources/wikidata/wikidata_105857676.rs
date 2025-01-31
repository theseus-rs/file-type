use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857676: FileFormat = FileFormat {
    id: 105_857_676,
    puid: "wikidata/105857676",
    name: "Infinity Engine Animation (v1)",
    extensions: &["bam"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x41, 0x4D, 0x20, 0x56, 0x31, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
