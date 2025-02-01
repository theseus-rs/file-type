use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205580: FileFormat = FileFormat {
    id: 28_205_580,
    puid: "wikidata/28205580",
    name: "OS/2 Icon",
    extensions: &["ico"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
