use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113674366: FileFormat = FileFormat {
    id: 113_674_366,
    puid: "wikidata/113674366",
    name: "Final Draft 5-7 Template",
    extensions: &["fdt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
