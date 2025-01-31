use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_81526528: FileFormat = FileFormat {
    id: 81_526_528,
    puid: "wikidata/81526528",
    name: "MicroStation Resource data",
    extensions: &["dat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
