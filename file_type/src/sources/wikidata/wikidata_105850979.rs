use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850979: FileFormat = FileFormat {
    id: 105_850_979,
    source_type: SourceType::Wikidata,
    name: "ST-6 color Table",
    extensions: &["tbl"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x54, 0x2D, 0x36, 0x20, 0x43, 0x6F, 0x6C, 0x6F, 0x72, 0x20, 0x54, 0x61,
                    0x62, 0x6C, 0x65, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
