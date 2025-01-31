use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_8041702: FileFormat = FileFormat {
    id: 8_041_702,
    puid: "wikidata/8041702",
    name: "eXtended Binary",
    extensions: &["xb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
