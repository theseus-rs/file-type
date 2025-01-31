use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28771221: FileFormat = FileFormat {
    id: 28_771_221,
    puid: "wikidata/28771221",
    name: "MAT-file, Level 5, version 7.3",
    extensions: &["mat"],
    media_types: &["application/x-matlab-data"],
    internal_signatures: &[],
    related_formats: &[],
};
