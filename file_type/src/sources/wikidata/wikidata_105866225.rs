use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866225: FileFormat = FileFormat {
    id: 105_866_225,
    source_type: SourceType::Wikidata,
    name: "Rhea-PV2D (PlotView-2D) plot",
    extensions: &["plt"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x53, 0x45, 0x43, 0x54, 0x49, 0x4F, 0x4E, 0x20, 0x48, 0x45, 0x41, 0x44,
                    0x45, 0x52, 0x5D, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
