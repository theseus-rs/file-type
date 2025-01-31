use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27473615: FileFormat = FileFormat {
    id: 27_473_615,
    puid: "wikidata/27473615",
    name: "Band Interleaved by Line Image File",
    extensions: &["bil"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
