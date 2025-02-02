use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28770336: FileFormat = FileFormat {
    id: 28_770_336,
    source_type: SourceType::Wikidata,
    name: "Lotus 1-2-3 Chart",
    extensions: &["pic"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
