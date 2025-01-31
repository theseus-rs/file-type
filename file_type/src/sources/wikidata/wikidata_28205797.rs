use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205797: FileFormat = FileFormat {
    id: 28_205_797,
    puid: "wikidata/28205797",
    name: "Canvas Picture Format",
    extensions: &["cnv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
