use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_65533440: FileFormat = FileFormat {
    id: 65_533_440,
    puid: "wikidata/65533440",
    name: "BigOven Recipe Box File",
    extensions: &["crb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
