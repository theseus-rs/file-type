use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27966975: FileFormat = FileFormat {
    id: 27_966_975,
    puid: "wikidata/27966975",
    name: "WSR",
    extensions: &["wsr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
