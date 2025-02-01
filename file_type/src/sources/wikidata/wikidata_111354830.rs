use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111354830: FileFormat = FileFormat {
    id: 111_354_830,
    puid: "wikidata/111354830",
    name: "Yamaha Tyros 2 custom drum voice file",
    extensions: &["tvd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
