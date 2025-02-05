use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857345: FileFormat = FileFormat {
    id: 105_857_345,
    source_type: SourceType::Wikidata,
    name: "WITCH-DOS Job Control",
    extensions: &["joc"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x57, 0x49, 0x54, 0x43, 0x48, 0x2D, 0x44, 0x4F, 0x53,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
