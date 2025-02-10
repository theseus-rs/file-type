use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865831: FileFormat = FileFormat {
    id: 105_865_831,
    source_type: SourceType::Wikidata,
    name: "FoxPro compressed dist. archive (additional parts)",
    extensions: &["pa1", "pa2"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x50, 0x41, 0x43, 0x46, 0x50, 0x50, 0x46,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
