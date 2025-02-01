use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131163238: FileFormat = FileFormat {
    id: 131_163_238,
    puid: "wikidata/131163238",
    name: "Stan model file",
    extensions: &["stan"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
