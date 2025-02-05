use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864048: FileFormat = FileFormat {
    id: 105_864_048,
    source_type: SourceType::Wikidata,
    name: "MarxMenu script (with rem)",
    extensions: &["mnu"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x6F, 0x6D, 0x6D, 0x65, 0x6E, 0x74, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
