use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206749: FileFormat = FileFormat {
    id: 28_206_749,
    puid: "wikidata/28206749",
    name: "Native Image File Format",
    extensions: &["niff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
