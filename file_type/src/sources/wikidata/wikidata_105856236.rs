use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856236: FileFormat = FileFormat {
    id: 105_856_236,
    puid: "wikidata/105856236",
    name: "Microsoft App-V Sequencer Differential SFT",
    extensions: &["dsft"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x53, 0x46, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
