use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205779: FileFormat = FileFormat {
    id: 28_205_779,
    puid: "wikidata/28205779",
    name: "Bob ray tracer bitmap",
    extensions: &["bob"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
