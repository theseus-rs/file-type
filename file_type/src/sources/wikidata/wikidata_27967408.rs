use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967408: FileFormat = FileFormat {
    id: 27_967_408,
    puid: "wikidata/27967408",
    name: "Codisk Audio File",
    extensions: &["caf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
