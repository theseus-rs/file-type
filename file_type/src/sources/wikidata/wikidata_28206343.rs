use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206343: FileFormat = FileFormat {
    id: 28_206_343,
    source_type: SourceType::Wikidata,
    name: "ImgStar",
    extensions: &["cpx", "flt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
