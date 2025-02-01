use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_65990735: FileFormat = FileFormat {
    id: 65_990_735,
    puid: "wikidata/65990735",
    name: "Adobe Premiere Library",
    extensions: &["plb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
