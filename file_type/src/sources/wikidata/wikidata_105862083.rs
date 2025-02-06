use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862083: FileFormat = FileFormat {
    id: 105_862_083,
    source_type: SourceType::Wikidata,
    name: "Meta Raster Format XML metadata",
    extensions: &["mrf"],
    media_types: &["text/xml"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x4D, 0x52, 0x46, 0x5F, 0x4D, 0x45, 0x54, 0x41, 0x3E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
