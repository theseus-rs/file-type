use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105867626: FileFormat = FileFormat {
    id: 105_867_626,
    puid: "wikidata/105867626",
    name: "Nintendo Switch Submission Zipped Package",
    extensions: &["nsz"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x46, 0x53, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
