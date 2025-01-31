use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28804253: FileFormat = FileFormat {
    id: 28_804_253,
    puid: "wikidata/28804253",
    name: "Uniform Office Format",
    extensions: &["eof"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
