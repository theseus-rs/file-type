use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111568387: FileFormat = FileFormat {
    id: 111_568_387,
    puid: "wikidata/111568387",
    name: "Managed Object Format",
    extensions: &["mof"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
