use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205835: FileFormat = FileFormat {
    id: 28_205_835,
    puid: "wikidata/28205835",
    name: "Cloé picture",
    extensions: &["clo", "cloe"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
