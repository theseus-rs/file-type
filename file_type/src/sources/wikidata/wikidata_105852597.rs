use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852597: FileFormat = FileFormat {
    id: 105_852_597,
    source_type: SourceType::Wikidata,
    name: "drububu.com STereoLithography (binary)",
    extensions: &["stl"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x20, 0x67, 0x65, 0x6E, 0x65, 0x72, 0x61, 0x74, 0x65, 0x64, 0x20, 0x61,
                    0x74, 0x20, 0x64, 0x72, 0x75, 0x62, 0x75, 0x62, 0x75, 0x2E, 0x63, 0x6F, 0x6D,
                    0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
