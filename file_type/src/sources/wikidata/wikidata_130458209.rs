use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130458209: FileFormat = FileFormat {
    id: 130_458_209,
    puid: "wikidata/130458209",
    name: "Pan source code file",
    extensions: &["pan"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
