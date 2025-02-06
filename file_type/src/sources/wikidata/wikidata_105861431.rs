use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861431: FileFormat = FileFormat {
    id: 105_861_431,
    source_type: SourceType::Wikidata,
    name: "Sothink Logo Maker logo",
    extensions: &["lmk"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x09, 0x00, 0x00, 0x00, 0x4C, 0x6F, 0x67, 0x6F, 0x4D, 0x61, 0x6B, 0x65, 0x72,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
