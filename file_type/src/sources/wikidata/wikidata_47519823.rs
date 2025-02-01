use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47519823: FileFormat = FileFormat {
    id: 47_519_823,
    puid: "wikidata/47519823",
    name: "Serif PagePlus Publication file format, version 6",
    extensions: &["ppp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
