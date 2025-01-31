use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852885: FileFormat = FileFormat {
    id: 105_852_885,
    puid: "wikidata/105852885",
    name: "SPSS for Windows Data (IBM)",
    extensions: &["sav"],
    media_types: &["application/spss-sav"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x24, 0x46, 0x4C, 0x32, 0x40, 0x28, 0x23, 0x29, 0x20, 0x49, 0x42, 0x4D, 0x20,
                    0x53, 0x50, 0x53, 0x53, 0x20, 0x53, 0x54, 0x41, 0x54, 0x49, 0x53, 0x54, 0x49,
                    0x43, 0x53, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
