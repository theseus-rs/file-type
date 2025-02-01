use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28771220: FileFormat = FileFormat {
    id: 28_771_220,
    puid: "wikidata/28771220",
    name: "MAT-file, Level 5, version 7",
    extensions: &["mat"],
    media_types: &["application/x-matlab-data"],
    internal_signatures: &[],
    related_formats: &[],
};
