use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28770329: FileFormat = FileFormat {
    id: 28_770_329,
    source_type: SourceType::Wikidata,
    name: "Lightwave 3D Object",
    extensions: &["lwo"],
    media_types: &["application/x-lightwave", "image/x-lwo"],
    internal_signatures: &[],
    related_formats: &[],
};
