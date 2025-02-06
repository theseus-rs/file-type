use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857068: FileFormat = FileFormat {
    id: 105_857_068,
    source_type: SourceType::Wikidata,
    name: "FlashPrint XG-Code",
    extensions: &["gx"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x78, 0x67, 0x63, 0x6F, 0x64, 0x65, 0x20, 0x31, 0x2E, 0x30, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
