use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128123256: FileFormat = FileFormat {
    id: 128_123_256,
    puid: "wikidata/128123256",
    name: "Stylus file format",
    extensions: &["styl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
