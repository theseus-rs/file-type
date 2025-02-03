use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853718: FileFormat = FileFormat {
    id: 105_853_718,
    source_type: SourceType::Wikidata,
    name: "VCOM Web Easy album",
    extensions: &["alb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
