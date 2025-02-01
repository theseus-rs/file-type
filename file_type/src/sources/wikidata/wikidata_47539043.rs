use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47539043: FileFormat = FileFormat {
    id: 47_539_043,
    puid: "wikidata/47539043",
    name: "AutoCAD dbConnect Query Set",
    extensions: &["dbq"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
