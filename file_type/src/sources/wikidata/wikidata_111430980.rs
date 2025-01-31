use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111430980: FileFormat = FileFormat {
    id: 111_430_980,
    puid: "wikidata/111430980",
    name: "ExtendScript Script File",
    extensions: &["jxs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
