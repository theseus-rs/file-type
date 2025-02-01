use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1194435: FileFormat = FileFormat {
    id: 1_194_435,
    puid: "wikidata/1194435",
    name: "MPEG-2 transport stream",
    extensions: &["ts", "tsa", "tsv"],
    media_types: &["video/mp2t", "video/mp2t", "video/mp2t"],
    internal_signatures: &[],
    related_formats: &[],
};
