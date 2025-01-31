use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51994105: FileFormat = FileFormat {
    id: 51_994_105,
    puid: "wikidata/51994105",
    name: "IBM DisplayWrite Revisable Form Text File",
    extensions: &["rft"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
