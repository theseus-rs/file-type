use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28600260: FileFormat = FileFormat {
    id: 28_600_260,
    puid: "wikidata/28600260",
    name: "AWD",
    extensions: &["awd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
