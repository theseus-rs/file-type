use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28975824: FileFormat = FileFormat {
    id: 28_975_824,
    puid: "wikidata/28975824",
    name: "Lightwave 3D Layered Object",
    extensions: &["lwo"],
    media_types: &["application/x-lightwave"],
    internal_signatures: &[],
    related_formats: &[],
};
