use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112819749: FileFormat = FileFormat {
    id: 112_819_749,
    puid: "wikidata/112819749",
    name: "Detailer 3D File",
    extensions: &["vdu"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
