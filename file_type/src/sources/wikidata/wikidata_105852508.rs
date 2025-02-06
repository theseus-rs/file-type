use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852508: FileFormat = FileFormat {
    id: 105_852_508,
    source_type: SourceType::Wikidata,
    name: "SOSI map data (UTF8)",
    extensions: &["sos"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xEF, 0xBB, 0xBF, 0x2E, 0x48, 0x4F, 0x44, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
