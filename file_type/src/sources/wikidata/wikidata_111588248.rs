use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111588248: FileFormat = FileFormat {
    id: 111_588_248,
    puid: "wikidata/111588248",
    name: "Adobe InDesign Interchange Document",
    extensions: &["inx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
