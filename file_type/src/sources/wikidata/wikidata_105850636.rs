use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850636: FileFormat = FileFormat {
    id: 105_850_636,
    source_type: SourceType::Wikidata,
    name: "ClickFORMS data",
    extensions: &["clk"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5F, 0x43, 0x4C, 0x49, 0x43, 0x4B, 0x46, 0x4F, 0x52, 0x4D, 0x53,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
