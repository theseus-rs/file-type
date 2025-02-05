use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967215: FileFormat = FileFormat {
    id: 27_967_215,
    source_type: SourceType::Wikidata,
    name: "SCC Blaffer NT music",
    extensions: &["sbm"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x42, 0x6C, 0x61, 0x66, 0x20, 0x4E, 0x54, 0x20, 0x53, 0x6F, 0x6E, 0x67, 0x20,
                    0x20, 0x20, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
