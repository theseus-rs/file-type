use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853718: FileFormat = FileFormat {
    id: 105_853_718,
    source_type: SourceType::Wikidata,
    name: "VCOM Web Easy album",
    extensions: &["alb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
