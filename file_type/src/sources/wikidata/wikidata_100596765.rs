use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_100596765: FileFormat = FileFormat {
    id: 100_596_765,
    puid: "wikidata/100596765",
    name: "Minitab Project, version 12-13",
    extensions: &["mpj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
