use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207227: FileFormat = FileFormat {
    id: 28_207_227,
    puid: "wikidata/28207227",
    name: "RIFF DIB",
    extensions: &["rdi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
