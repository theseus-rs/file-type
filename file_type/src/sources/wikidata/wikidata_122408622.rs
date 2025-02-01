use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122408622: FileFormat = FileFormat {
    id: 122_408_622,
    puid: "wikidata/122408622",
    name: "68K Symbol File",
    extensions: &["sym"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
