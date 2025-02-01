use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206593: FileFormat = FileFormat {
    id: 28_206_593,
    puid: "wikidata/28206593",
    name: "MSX Interchange Format",
    extensions: &["mif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
