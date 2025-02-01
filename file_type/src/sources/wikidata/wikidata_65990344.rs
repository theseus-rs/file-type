use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_65990344: FileFormat = FileFormat {
    id: 65_990_344,
    puid: "wikidata/65990344",
    name: "Adobe Premiere Project",
    extensions: &["ppj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
