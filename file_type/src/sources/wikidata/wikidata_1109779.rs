use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1109779: FileFormat = FileFormat {
    id: 1_109_779,
    source_type: SourceType::Wikidata,
    name: "file shortcut",
    extensions: &["desktop", "lnk"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4C, 0x00, 0x00, 0x00, 0x01, 0x14, 0x02, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
