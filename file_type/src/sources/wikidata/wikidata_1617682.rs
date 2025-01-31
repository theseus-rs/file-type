use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1617682: FileFormat = FileFormat {
    id: 1_617_682,
    puid: "wikidata/1617682",
    name: "DVD BUP File",
    extensions: &["bup"],
    media_types: &["video/dvd"],
    internal_signatures: &[],
    related_formats: &[],
};
