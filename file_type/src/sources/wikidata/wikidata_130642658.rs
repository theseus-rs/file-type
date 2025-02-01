use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130642658: FileFormat = FileFormat {
    id: 130_642_658,
    puid: "wikidata/130642658",
    name: "Robot Framework file format",
    extensions: &["robot"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
