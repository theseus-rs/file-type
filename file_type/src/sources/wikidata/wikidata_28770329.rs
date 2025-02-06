use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28770329: FileFormat = FileFormat {
    id: 28_770_329,
    source_type: SourceType::Wikidata,
    name: "Lightwave 3D Object",
    extensions: &["lwo"],
    media_types: &["application/x-lightwave", "image/x-lwo"],
    signatures: &[],
    related_formats: &[],
};
