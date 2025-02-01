use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113674320: FileFormat = FileFormat {
    id: 113_674_320,
    puid: "wikidata/113674320",
    name: "Final Draft 8 Template",
    extensions: &["fdxt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
