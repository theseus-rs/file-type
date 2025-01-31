use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28770728: FileFormat = FileFormat {
    id: 28_770_728,
    puid: "wikidata/28770728",
    name: "MAT-file, Level 5, version 6",
    extensions: &["mat"],
    media_types: &["application/x-matlab-data"],
    internal_signatures: &[],
    related_formats: &[],
};
