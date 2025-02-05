use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856236: FileFormat = FileFormat {
    id: 105_856_236,
    source_type: SourceType::Wikidata,
    name: "Microsoft App-V Sequencer Differential SFT",
    extensions: &["dsft"],
    media_types: &[],
    signatures: &[Signature {
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
