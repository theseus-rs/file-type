use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28771107: FileFormat = FileFormat {
    id: 28_771_107,
    puid: "wikidata/28771107",
    name: "MAT-file, Level 4",
    extensions: &["mat"],
    media_types: &["application/x-matlab-data"],
    internal_signatures: &[],
    related_formats: &[],
};
