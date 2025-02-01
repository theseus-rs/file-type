use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47538977: FileFormat = FileFormat {
    id: 47_538_977,
    puid: "wikidata/47538977",
    name: "AutoCAD Template Menu File",
    extensions: &["mnu"],
    media_types: &["application/x-autocad"],
    internal_signatures: &[],
    related_formats: &[],
};
