use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28975824: FileFormat = FileFormat {
    id: 28_975_824,
    source_type: SourceType::Wikidata,
    name: "Lightwave 3D Layered Object",
    extensions: &["lwo"],
    media_types: &["application/x-lightwave"],
    signatures: &[],
    related_formats: &[],
};
