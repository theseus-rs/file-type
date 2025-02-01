use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_67124083: FileFormat = FileFormat {
    id: 67_124_083,
    puid: "wikidata/67124083",
    name: "Print Artist label file format",
    extensions: &["lbl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
