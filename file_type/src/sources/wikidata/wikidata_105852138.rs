use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852138: FileFormat = FileFormat {
    id: 105_852_138,
    source_type: SourceType::Wikidata,
    name: "SatHawk data file",
    extensions: &["sat"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x53, 0x61, 0x74, 0x48,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
